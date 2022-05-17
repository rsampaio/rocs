use rocl::apis::catalog_api::catalog_get;
use rocl::apis::configuration::Configuration;
use rocl::models::Schemas;
use std::collections::HashMap;

mod catalog;
pub use catalog::catalog;

mod service_instance;
pub use service_instance::{deprovision, info, provision};

mod service_binding;
pub use service_binding::{bind, unbind};

pub const USER_AGENT: &str = "ROCS v0.2";
pub const DEFAULT_API_VERSION: &str = "2.15";
pub const POOL_INTERVAL: u64 = 5;

pub struct Options {
    pub json_output: bool,
    pub curl_output: bool,
    pub synchronous: bool,
}

pub fn generate_curl_command(
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
    let schemas: Schemas = Schemas::new();

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
