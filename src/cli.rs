use prettytable::{format, Table};
use rocl::apis::client::APIClient;
use rocl::models::{Schemas, ServiceBindingRequest, ServiceInstanceProvisionRequest};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};
use uuid::Uuid;
use valico::json_schema;

use models::{ServiceBindingOutput, ServiceInstanceOutput};

pub const USER_AGENT: &str = "ROCS v0.2";
const DEFAULT_API_VERSION: &str = "2.15";

pub const POOL_INTERVAL: u64 = 5;

pub struct Options {
    pub json_output: bool,
    pub curl_output: bool,
    pub synchronous: bool,
}

pub fn catalog(
    _: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let catalog_api = client.catalog_api();
    let catalog = catalog_api
        .catalog_get(DEFAULT_API_VERSION)
        .expect("catalog request failed");

    if options.curl_output {
        println!(
            "{}",
            generate_curl_command(
                "catalog".to_owned(),
                "GET".to_owned(),
                "".to_owned(),
                false,
                "".to_owned(),
                "".to_owned()
            )
        );
        return Ok(());
    }

    match options.json_output {
        false => {
            let mut services_table = Table::new();

            services_table.add_row(row!["Service", "Description", "Plans", "Extensions"]);

            for s in catalog.services.unwrap().iter() {
                let mut plans_table = Table::new();
                let mut extensions_table = Table::new();

                plans_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

                for p in s.plans.iter() {
                    plans_table.add_row(row![p.name]);
                }

                if let Some(extensions) = &s.extensions {
                    for p in extensions.iter() {
                        extensions_table.add_row(row![p.id]);
                    }
                }

                services_table.add_row(row![s.name, s.description, plans_table]);
            }

            services_table.printstd();
        }
        true => {
            println!(
                "{}",
                serde_json::to_string(&catalog.services.unwrap()).unwrap()
            );
        }
    };

    Ok(())
}

pub fn deprovision(
    matches: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let instance_id = matches.value_of("instance").unwrap().to_string();

    if options.curl_output {
        println!(
            "{}",
            generate_curl_command(
                "service_instance".to_owned(),
                "DELETE".to_owned(),
                "".to_owned(),
                options.synchronous,
                instance_id,
                "".to_owned()
            )
        );
        return Ok(());
    }

    let si_api = client.service_instances_api();
    si_api
        .service_instance_deprovision(
            DEFAULT_API_VERSION,
            &*instance_id,
            "",
            "",
            USER_AGENT,
            !options.synchronous,
        )
        .expect("deprovisioning request failed");
    Ok(())
}

pub fn provision(
    matches: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let service = matches.value_of("service").unwrap().to_string();
    let plan = matches.value_of("plan").unwrap().to_string();

    let si_api = client.service_instances_api();

    let (service_id, plan_id, schemas) =
        find_service_plan_id(&client, service, plan).expect("service or plan id not found");

    let mut provision_request = ServiceInstanceProvisionRequest::new(
        service_id.clone(),
        plan_id.clone(),
        String::from(""),
        String::from(""),
    );

    let parameters = matches.values_of("parameters");
    let context = matches.values_of("context");

    match schemas {
        Some(s) => match validate_schema(
            s.service_instance.unwrap().create.unwrap(),
            parameters.clone(),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        },
        None => {}
    }

    provision_request.parameters = Some(json!(parse_parameters(parameters).unwrap()));
    provision_request.context = Some(json!(parse_parameters(context).unwrap()));

    let instance_id = Uuid::new_v4().to_hyphenated().to_string();

    if options.curl_output {
        println!(
            "{}",
            generate_curl_command(
                "service_instance".to_owned(),
                "PUT".to_owned(),
                serde_json::to_string_pretty(&provision_request).unwrap(),
                options.synchronous,
                instance_id,
                "".to_owned()
            )
        );
        return Ok(());
    }

    let _provision_response = si_api
        .service_instance_provision(
            DEFAULT_API_VERSION,
            &*instance_id, // from String to &str
            provision_request,
            USER_AGENT,
            !options.synchronous, // Accepts-incomplete
        )
        .expect("provision request failed");

    if matches.is_present("wait") {
        eprintln!(
            "[INFO] waiting service instance {} provisioning",
            instance_id
        );

        loop {
            thread::sleep(time::Duration::new(POOL_INTERVAL, 0));

            let last_op = si_api.service_instance_last_operation_get(
                DEFAULT_API_VERSION,
                &*instance_id,
                &*service_id, // service_id
                &*plan_id,    // plan id
                "",           // operation
            );

            if let Ok(lo) = last_op {
                match lo.state {
                    rocl::models::State::InProgress => continue,
                    _ => break,
                }
            }
        }
    }

    let provisioned_instance = si_api
        .service_instance_get(DEFAULT_API_VERSION, &*instance_id, USER_AGENT, "", "")
        .expect("service instance fetch failed");

    match options.json_output {
        false => {
            let mut table = Table::new();
            table.add_row(row!["Instance ID", "Dashboard URL"]);
            table.add_row(row![
                &*instance_id,
                provisioned_instance.dashboard_url.unwrap()
            ]);
            table.printstd();
        }
        true => {
            let si_out = ServiceInstanceOutput {
                service_instance_id: Some(instance_id),
                service_instance_resource: Some(provisioned_instance),
            };

            println!("{}", serde_json::to_string(&si_out).unwrap());
        }
    };

    Ok(())
}

pub fn bind(
    matches: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let binding_api = client.service_bindings_api();

    let mut binding_id = Uuid::new_v4().to_hyphenated().to_string();
    let instance_id = matches.value_of("instance").unwrap().to_string();

    // bindings
    // if binding id is present, just fetch the id
    if matches.is_present("binding") {
        binding_id = matches.value_of("binding").unwrap().into();
    } else {
        let mut binding_request = ServiceBindingRequest::new("".into(), "".into());
        let parameters = matches.values_of("parameters");
        let context = matches.values_of("context");

        // - fetch service_instance
        //   - extract service_id
        //   - extract plan_id
        // - fetch catalog
        //   - extract schemas
        /*
        match schemas {
            Some(s) => match validate_schema(
                s.service_binding.unwrap().create.unwrap(),
                parameters.clone(),
            ) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            None => {}
        }
         */

        binding_request.parameters = Some(json!(parse_parameters(parameters).unwrap()));
        binding_request.context = Some(json!(parse_parameters(context).unwrap()));

        if options.curl_output {
            println!(
                "{}",
                generate_curl_command(
                    "service_binding".to_owned(),
                    "PUT".to_owned(),
                    serde_json::to_string_pretty(&binding_request).unwrap(),
                    options.synchronous,
                    instance_id,
                    binding_id,
                )
            );
            return Ok(());
        }

        let binding_response = binding_api
            .service_binding_binding(
                DEFAULT_API_VERSION,
                &*instance_id,
                &*binding_id,
                binding_request,
                USER_AGENT,
                !options.synchronous,
            )
            .expect("binding failed");

        if options.synchronous || !matches.is_present("wait") {
            match options.json_output {
                true => {
                    let mut binding_output = HashMap::new();
                    binding_output.insert(binding_id, &binding_response);
                    println!("{}", serde_json::to_string(&binding_output).unwrap());
                }
                false => {
                    let mut table = Table::new();
                    table.add_row(row!["Instance ID", "Binding ID"]);
                    table.add_row(row![&*instance_id, &*binding_id]);
                    table.printstd();
                }
            }
            return Ok(());
        }

        if matches.is_present("wait") {
            eprintln!("[INFO] waiting binding {} provisioning", binding_id);

            loop {
                thread::sleep(time::Duration::new(POOL_INTERVAL, 0));

                let last_op = binding_api.service_binding_last_operation_get(
                    DEFAULT_API_VERSION,
                    &*instance_id,
                    &*binding_id,
                    "", // service_id
                    "", // plan id
                    "", // operation
                );

                if let Ok(lo) = last_op {
                    match lo.state {
                        rocl::models::State::InProgress => continue,
                        _ => break,
                    }
                }
            }
        }
    }

    let provisioned_binding = binding_api
        .service_binding_get(
            DEFAULT_API_VERSION,
            &*instance_id,
            &*binding_id,
            USER_AGENT,
            "",
            "",
        )
        .expect("service binding fetch failed");

    match options.json_output {
        false => {
            let mut table = Table::new();
            table.add_row(row!["Instance ID"]);
            table.add_row(row![&*instance_id]);
            table.add_row(row!["Binding ID"]);
            table.add_row(row![&*binding_id]);
            table.add_row(row!["Credentials"]);
            table.add_row(row![serde_json::to_string_pretty(&provisioned_binding)?]);
            table.printstd();
        }
        true => {
            let sb_out = ServiceBindingOutput {
                service_binding_id: Some(binding_id),
                service_binding_resource: Some(provisioned_binding),
            };
            println!("{}", serde_json::to_string(&sb_out).unwrap());
        }
    };

    Ok(())
}

pub fn unbind(
    matches: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let binding_api = client.service_bindings_api();

    let instance_id = matches.value_of("instance").unwrap().to_string();
    let binding_id = matches.value_of("binding").unwrap().to_string();

    if options.curl_output {
        println!(
            "{}",
            generate_curl_command(
                "service_binding".to_owned(),
                "DELETE".to_owned(),
                "".to_owned(),
                options.synchronous,
                instance_id,
                binding_id
            )
        );
        return Ok(());
    }

    let _unbinding_response = binding_api
        .service_binding_unbinding(
            DEFAULT_API_VERSION,
            &*instance_id,
            &*binding_id,
            "", // service_id
            "", // plan_id
            USER_AGENT,
            !options.synchronous,
        )
        .expect("service binding unbind failed");

    Ok(())
}

fn generate_curl_command(
    object: String,
    method: String,
    body: String,
    synchronous: bool,
    sid: String,
    bid: String,
) -> String {
    let sync_opt = if !synchronous {
        "?accepts_incomplete=true"
    } else {
        ""
    };

    let request_body = if body != "" {
        format!(" \\\n-d '{}'", body)
    } else {
        format!("")
    };

    let path = match object.as_str() {
        "catalog" => format!("catalog"),
        "service_instance" => format!("service_instances/{}", sid),
        "service_binding" => format!("service_instances/{}/service_bindings/{}", sid, bid),
        _ => {
            println!("error generating path, object is invalid");
            "".into()
        }
    };

    let curl_command = format!(
        "curl -X {method} -u {userpass} {url}/{version}/{path}{sync_opt}{body}",
        userpass = "$ROCS_BROKER_USERNAME:$ROCS_BROKER_PASSWORD",
        url = "$ROCS_BROKER_URL",
        method = method,
        version = "v2",
        path = path,
        sync_opt = sync_opt,
        body = request_body,
    );

    String::from(curl_command)
}

fn find_service_plan_id(
    client: &APIClient,
    service: String,
    plan: String,
) -> Result<(String, String, Option<Schemas>), &'static str> {
    let si_api = client.catalog_api();
    let catalog = si_api
        .catalog_get(DEFAULT_API_VERSION)
        .expect("failed to fetch catalog");

    let mut service_id = String::from("");
    let mut plan_id = String::from("");
    let mut schemas = None;

    'outer: for s in catalog.services.unwrap() {
        if s.name == service {
            service_id = s.id;
            for p in s.plans {
                if p.name == plan {
                    plan_id = p.id;
                    schemas = p.schemas;
                    break 'outer;
                }
            }
        }
    }

    if plan_id == "" || service_id == "" {
        return Err("plan or service not found");
    }

    Ok((service_id, plan_id, schemas))
}

fn parse_parameters(params: Option<clap::Values>) -> Result<HashMap<String, String>, &'static str> {
    let mut parsed_params: HashMap<String, String> = HashMap::new();

    match params {
        Some(param) => {
            for kv in param {
                let keyvalue: Vec<&str> = kv.splitn(2, '=').collect();
                if keyvalue.len() < 2 {
                    println!("{} does not match format key=value, ignoring", kv);
                    continue;
                }
                parsed_params.insert(keyvalue[0].to_string(), keyvalue[1].to_string());
            }
        }
        None => {}
    }
    Ok(parsed_params)
}

fn validate_schema(
    schema: rocl::models::SchemaParameters,
    parameters: Option<clap::Values>,
) -> Result<(), Box<dyn Error>> {
    let mut scope = json_schema::Scope::new();

    let schema = scope
        .compile_and_return(schema.parameters.unwrap(), false)
        .unwrap();

    let validation = schema.validate(&json!(parse_parameters(parameters).unwrap()));

    match validation.is_valid() {
        false => {
            for err in validation.errors {
                println!("invalid parameter: {}", err.get_path());
            }
            return Err("schema validation failed".into());
        }
        true => return Ok(()),
    }
}
