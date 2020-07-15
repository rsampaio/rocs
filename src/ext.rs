use cli;
use openapi;
use reqwest;
use rocl::apis::client::APIClient;
use std::error::Error;

pub fn client(
    args: &clap::ArgMatches,
    client: APIClient,
    _options: cli::Options,
    global_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut extension_uri: String = "".into();

    let base_path: String = global_matches.value_of("broker_url").unwrap().to_owned();
    let broker_username: String = global_matches.value_of("broker_user").unwrap().to_owned();
    let broker_password: String = global_matches.value_of("broker_pass").unwrap().to_owned();

    let instance_id: String = args.value_of("instance").unwrap().to_owned();
    let path: String = args.value_of("path").unwrap().to_owned();

    let ext_client = reqwest::Client::new();

    let service_instance_api = client.service_instances_api();
    let service_instance = service_instance_api
        .service_instance_get(
            cli::DEFAULT_API_VERSION,
            &*instance_id,
            cli::USER_AGENT,
            "",
            "",
        )
        .expect("failed to retrieve service instance information");

    let service_id: String = service_instance.service_id.unwrap();

    let catalog_api = client.catalog_api();
    let catalog = catalog_api
        .catalog_get(cli::DEFAULT_API_VERSION)
        .expect("catalog request failed");

    'extension_search: for s in catalog.services.unwrap().iter() {
        if s.id == service_id {
            if let Some(extensions) = &s.extensions {
                for p in extensions.iter() {
                    if p.path == path {
                        extension_uri = format!("{}{}", base_path, p.openapi_url);
                        break 'extension_search;
                    }
                }
            }
        }
    }

    if extension_uri == String::from("") {
        return Err(Box::from("extension or path not found"));
    }

    let schema_text: String = ext_client
        .get(&extension_uri)
        .basic_auth(broker_username, Some(broker_password))
        .send()?
        .text()?;

    let schema = openapi::from_reader(schema_text.as_bytes())?;

    println!("{:#?}", schema.paths.contains_key(&*path));

    Ok(())
}
