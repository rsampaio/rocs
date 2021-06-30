use clap::ArgMatches;
use prettytable::{format, Table};
use rocl::{
    apis::{
        catalog_api::catalog_get,
        configuration::Configuration,
        service_bindings_api::{
            service_binding_binding, service_binding_get, service_binding_last_operation_get,
            service_binding_unbinding,
        },
        service_instances_api::{
            service_instance_deprovision, service_instance_get,
            service_instance_last_operation_get, service_instance_provision,
        },
    },
    models::{
        last_operation_resource::State, Schemas, ServiceBindingRequest,
        ServiceInstanceProvisionRequestBody,
    },
};
use serde_json::json;
use std::{collections::HashMap, error::Error};
use std::{thread, time};
use uuid::Uuid;
use valico::json_schema;

use crate::models::{ServiceBindingOutput, ServiceInstanceOutput};

pub const USER_AGENT: &str = "ROCS v0.2";
pub const DEFAULT_API_VERSION: &str = "2.15";
pub const POOL_INTERVAL: u64 = 5;

pub struct Options {
    pub json_output: bool,
    pub curl_output: bool,
    pub synchronous: bool,
}

pub async fn info(
    args: &ArgMatches<'_>,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let instance_id: String = args.value_of("instance").unwrap().to_owned();

    if options.curl_output {
        println!(
            "{}",
            generate_curl_command(
                "service_instance".to_owned(),
                "GET".to_owned(),
                "".to_owned(),
                false,
                instance_id,
                "".to_owned()
            )
        );
        return Ok(());
    }

    let si = service_instance_get(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id,
        Some(USER_AGENT),
        None,
        None,
    )
    .await
    .expect("failed to fetch service instance");

    match options.json_output {
        false => {
            let mut service_instance_table = Table::new();
            service_instance_table.add_row(row!["ID", "Service ID", "Plans ID", "Dashboard"]);
            service_instance_table.add_row(row![
                instance_id,
                si.service_id.unwrap(),
                si.plan_id.unwrap(),
                si.dashboard_url.unwrap()
            ]);
            service_instance_table.printstd();
        }
        true => {
            let mut instance_output = HashMap::new();
            instance_output.insert(instance_id, si);
            println!("{}", serde_json::to_string(&instance_output).unwrap());
        }
    };

    Ok(())
}

pub async fn catalog(
    _: &ArgMatches<'_>,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let catalog = catalog_get(&config, DEFAULT_API_VERSION)
        .await
        .expect("failed to get catalog");

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
                // let mut extensions_table = Table::new();

                plans_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

                for p in s.plans.iter() {
                    plans_table.add_row(row![p.name]);
                }

                /*
                if let Some(extensions) = &s.extensions {
                    extensions_table.add_row(row!["ID", "Path"]);

                    for p in extensions.iter() {
                        extensions_table.add_row(row![p.id, p.path]);
                    }
                }
                 */

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

pub async fn deprovision(
    matches: &ArgMatches<'_>,
    config: Configuration,
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

    service_instance_deprovision(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id,
        "",
        "",
        Some(USER_AGENT),
        Some(!options.synchronous),
    )
    .await
    .expect("deprovisioning request failed");
    Ok(())
}

pub async fn provision(
    matches: &ArgMatches<'_>,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let service = matches.value_of("service").unwrap().to_string();
    let plan = matches.value_of("plan").unwrap().to_string();

    let (service_id, plan_id, schemas) = find_service_plan_id(config.clone(), service, plan)
        .await
        .expect("service or plan id not found");

    let mut provision_request = ServiceInstanceProvisionRequestBody::new(
        service_id.clone(),
        plan_id.clone(),
        String::from(""),
        String::from(""),
    );

    let parameters = matches.values_of("parameters");
    let context = matches.values_of("context");

    //validate_service_schema(schemas, parameters.clone());

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

    let _provision_response = service_instance_provision(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id, // from String to &str
        provision_request,
        Some(USER_AGENT),
        Some(!options.synchronous), // Accepts-incomplete
    )
    .await
    .expect("provision request failed");

    if matches.is_present("wait") {
        eprintln!(
            "[INFO] waiting service instance {} provisioning",
            instance_id
        );

        loop {
            thread::sleep(time::Duration::new(POOL_INTERVAL, 0));

            let last_op = service_instance_last_operation_get(
                &config,
                DEFAULT_API_VERSION,
                &*instance_id,
                Some(&*service_id), // service_id
                Some(&*plan_id),    // plan id
                None,               // operation
            )
            .await
            .expect("failed to fetch last operation");

            match last_op.state {
                State::InProgress => continue,
                _ => break,
            }
        }
    }

    let provisioned_instance = service_instance_get(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id,
        Some(USER_AGENT),
        None,
        None,
    )
    .await
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

pub async fn bind(
    matches: &ArgMatches<'_>,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let mut binding_id = Uuid::new_v4().to_hyphenated().to_string();
    let instance_id = matches.value_of("instance").unwrap().to_string();

    // bindings
    // if binding id is present, just fetch the id
    if matches.is_present("binding") {
        binding_id = matches.value_of("binding").unwrap().into();
    }

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
        match matches.is_present("binding") {
            false => {
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
            }
            true => {
                println!(
                    "{}",
                    generate_curl_command(
                        "service_binding".to_owned(),
                        "GET".to_owned(),
                        serde_json::to_string_pretty(&binding_request).unwrap(),
                        options.synchronous,
                        instance_id,
                        binding_id,
                    )
                );
            }
        }

        let binding_response = service_binding_binding(
            &config,
            DEFAULT_API_VERSION,
            &*instance_id,
            &*binding_id,
            binding_request,
            Some(USER_AGENT),
            Some(!options.synchronous),
        )
        .await
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

                let last_op = service_binding_last_operation_get(
                    &config,
                    DEFAULT_API_VERSION,
                    &*instance_id,
                    &*binding_id,
                    None, // service_id
                    None, // plan id
                    None, // operation
                )
                .await
                .expect("failed to get binding last operation");

                match last_op.state {
                    State::InProgress => continue,
                    _ => break,
                }
            }
        }
    }

    let provisioned_binding = service_binding_get(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id,
        &*binding_id,
        Some(USER_AGENT),
        None,
        None,
    )
    .await
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

pub async fn unbind(
    matches: &ArgMatches<'_>,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
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

    let _unbinding_response = service_binding_unbinding(
        &config,
        DEFAULT_API_VERSION,
        &*instance_id,
        &*binding_id,
        "", // service_id
        "", // plan_id
        Some(USER_AGENT),
        Some(!options.synchronous),
    )
    .await
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
        "curl -H 'Content-type: application/json' -H 'X-Broker-API-Version: 2.16' -X {method} -u {userpass} {url}/{version}/{path}{sync_opt}{body}",
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

async fn find_service_plan_id(
    config: Configuration,
    service: String,
    plan: String,
) -> Result<(String, String, Schemas), &'static str> {
    let catalog = catalog_get(&config, DEFAULT_API_VERSION)
        .await
        .expect("failed to fetch catalog");

    let mut service_id = String::from("");
    let mut plan_id = String::from("");
    let mut schemas: Schemas = Schemas::new();

    'outer: for s in catalog.services.unwrap() {
        if s.name == service {
            service_id = s.id;
            for p in s.plans {
                if p.name == plan {
                    plan_id = p.id;
                    //schemas = *p.schemas.unwrap();
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

pub fn parse_parameters(
    params: Option<clap::Values>,
) -> Result<HashMap<String, String>, &'static str> {
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

fn validate_service_schema(
    schema: Schemas,
    parameters: Option<clap::Values>,
) -> Result<(), Box<dyn Error>> {
    let mut scope = json_schema::Scope::new();

    let schema = scope
        .compile_and_return(
            schema
                .service_instance
                .unwrap()
                .create
                .unwrap()
                .parameters
                .unwrap(),
            false,
        )
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
