use crate::cli::{
    generate_curl_command, parse_parameters, Options, DEFAULT_API_VERSION, POOL_INTERVAL,
    USER_AGENT,
};
use crate::models::ServiceBindingOutput;

use clap::ArgMatches;
use prettytable::Table;
use rocl::{
    apis::{
        configuration::Configuration,
        service_bindings_api::{
            service_binding_binding, service_binding_get, service_binding_last_operation_get,
            service_binding_unbinding,
        },
    },
    models::{last_operation_resource::State, ServiceBindingRequest},
};
use serde_json::json;
use std::{collections::HashMap, error::Error};
use std::{thread, time};
use uuid::Uuid;

pub async fn bind(
    matches: &ArgMatches,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let mut binding_id = Uuid::new_v4().as_hyphenated().to_string();
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
                        instance_id.clone(),
                        binding_id.clone(),
                    )
                );
            }
            true => {
                println!(
                    "{}",
                    generate_curl_command(
                        "service_binding".to_owned(),
                        "GET".to_owned(),
                        "".to_owned(),
                        options.synchronous,
                        instance_id.clone(),
                        binding_id.clone(),
                    )
                );
            }
        }
        return Ok(());
    }

    if !matches.is_present("binding") {
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
    matches: &ArgMatches,
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
