use crate::cli::{
    find_service_plan_id, generate_curl_command, parse_parameters, Options, DEFAULT_API_VERSION,
    POOL_INTERVAL, USER_AGENT,
};
use crate::models::ServiceInstanceOutput;

use clap::ArgMatches;
use prettytable::Table;
use rocl::{
    apis::{
        configuration::Configuration,
        service_instances_api::{
            service_instance_deprovision, service_instance_get,
            service_instance_last_operation_get, service_instance_provision,
        },
    },
    models::{last_operation_resource::State, ServiceInstanceProvisionRequestBody},
};
use serde_json::json;
use std::{collections::HashMap, error::Error};
use std::{thread, time};
use uuid::Uuid;

pub async fn info(
    args: &ArgMatches,
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

pub async fn deprovision(
    matches: &ArgMatches,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let instance_id = matches.value_of("instance").unwrap().to_string();

    // the spec requires this parameters
    let plan_id = matches.value_of("plan").unwrap_or("ignore").to_string();
    let service_id = matches.value_of("service").unwrap_or("ignore").to_string();

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
        &*plan_id,
        &*service_id,
        Some(USER_AGENT),
        Some(!options.synchronous),
    )
    .await
    .expect("deprovisioning request failed");
    Ok(())
}

pub async fn provision(
    matches: &ArgMatches,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let service = matches.value_of("service").unwrap().to_string();
    let plan = matches.value_of("plan").unwrap().to_string();

    let (service_id, plan_id, _schemas) = find_service_plan_id(config.clone(), service, plan)
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

    let instance_id = Uuid::new_v4().as_hyphenated().to_string();

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
