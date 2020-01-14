use prettytable::{format, Table};
use roc::apis::client::APIClient;
use roc::models::{ServiceBindingRequest, ServiceInstanceProvisionRequest};
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::error::Error;
use std::{thread, time};
use uuid::Uuid;

const USER_AGENT: &str = "ROC v0.1";
const DEFAULT_API_VERSION: &str = "2.15";

pub struct Options {
    pub json_output: bool,
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

    match options.json_output {
        false => {
            let mut services_table = Table::new();
            services_table.add_row(row!["Service", "Description", "Plans"]);

            for s in catalog.services.unwrap().iter() {
                let mut plans_table = Table::new();
                plans_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

                for p in s.plans.iter() {
                    plans_table.add_row(row![p.name]);
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
    let instance_id = matches.value_of("instance-id").unwrap().to_string();
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

    let (service_id, plan_id) =
        find_service_plan_id(&client, service, plan).expect("service or plan id not found");

    let mut provision_request = ServiceInstanceProvisionRequest::new(
        service_id.clone(),
        plan_id.clone(),
        String::from(""),
        String::from(""),
    );

    // TODO(rsampaio): receive parameters from caller
    provision_request.parameters = Some(json!({"region": "us-east-1"}));

    let instance_id = Uuid::new_v4().to_hyphenated().to_string();

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
        let sp = Spinner::new(Spinners::Point, "provisioning service instance".to_string());

        loop {
            let last_op = si_api.service_instance_last_operation_get(
                DEFAULT_API_VERSION,
                &*instance_id,
                &*service_id, // service_id
                &*plan_id,    // plan id
                "",           // operation
            );

            if let Ok(lo) = last_op {
                match lo.state {
                    roc::models::State::InProgress => {
                        thread::sleep(time::Duration::new(2, 0));
                    }
                    _ => break,
                }
            }
        }
        println!("");
        sp.stop();
    }

    let provisioned_instance = si_api
        .service_instance_get(DEFAULT_API_VERSION, &*instance_id, USER_AGENT, "", "")
        .expect("service instance fetch failed");

    let mut table = Table::new();
    table.add_row(row!["Instance ID", "Dashboard URL"]);
    table.add_row(row![
        &*instance_id,
        provisioned_instance.dashboard_url.unwrap()
    ]);
    table.printstd();

    Ok(())
}

pub fn bind(
    matches: &clap::ArgMatches,
    client: APIClient,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let binding_api = client.service_bindings_api();
    let instance_api = client.service_instances_api();

    let binding_id = Uuid::new_v4().to_hyphenated().to_string();
    let instance_id = matches.value_of("instance-id").unwrap().to_string();

    let service_instance_response = instance_api
        .service_instance_get(DEFAULT_API_VERSION, &*instance_id, USER_AGENT, "", "")
        .expect("service instance request failed");

    let (service_id, plan_id) = find_service_plan_id(
        &client,
        service_instance_response.service_id.unwrap(),
        service_instance_response.plan_id.unwrap(),
    )?;

    let binding_request = ServiceBindingRequest::new(service_id.clone(), plan_id.clone());
    let _binding_response = binding_api.service_binding_binding(
        DEFAULT_API_VERSION,
        &*instance_id,
        &*binding_id,
        binding_request,
        USER_AGENT,
        !options.synchronous,
    );

    if !options.synchronous {
        let sp = Spinner::new(Spinners::Point, "provisioning binding".to_string());

        loop {
            let last_op = binding_api.service_binding_last_operation_get(
                DEFAULT_API_VERSION,
                &*instance_id,
                &*binding_id,
                &*service_id, // service_id
                &*plan_id,    // plan id
                "",           // operation
            );

            if let Ok(lo) = last_op {
                match lo.state {
                    roc::models::State::InProgress => {
                        thread::sleep(time::Duration::new(2, 0));
                    }
                    _ => break,
                }
            }
        }
        println!("");
        sp.stop();
    }

    Ok(())
}
pub fn unbind(
    _matches: &clap::ArgMatches,
    _client: APIClient,
    _options: Options,
) -> Result<(), Box<dyn Error>> {
    Err(Box::from("not implemented"))
}
pub fn creds(
    _matches: &clap::ArgMatches,
    _client: APIClient,
    _options: Options,
) -> Result<(), Box<dyn Error>> {
    Err(Box::from("not implemented"))
}

fn find_service_plan_id(
    client: &APIClient,
    service: String,
    plan: String,
) -> Result<(String, String), &'static str> {
    let si_api = client.catalog_api();
    let catalog = si_api
        .catalog_get(DEFAULT_API_VERSION)
        .expect("failed to fetch catalog");

    let mut service_id = String::from("");
    let mut plan_id = String::from("");

    'outer: for s in catalog.services.unwrap() {
        if s.name == service {
            service_id = s.id;
            for p in s.plans {
                if p.name == plan {
                    plan_id = p.id;
                    break 'outer;
                }
            }
        }
    }

    if plan_id == "" || service_id == "" {
        return Err("plan or service not found");
    }

    Ok((service_id, plan_id))
}
