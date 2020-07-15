extern crate clap;
extern crate rocl;
extern crate rocs;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
use rocl::apis::client::APIClient;
use rocl::apis::configuration::Configuration;
use rocs::cli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rocs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Rust OSB Client 'Super'")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("broker_url")
                .short("b")
                .long("broker")
                .env("ROCS_BROKER_URL")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("broker_user")
                .short("u")
                .long("username")
                .env("ROCS_BROKER_USERNAME")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("broker_pass")
                .short("a")
                .long("password")
                .env("ROCS_BROKER_PASSWORD")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("json")
                .help("Prints result in JSON format")
                .long("json")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("sync")
                .help("Execute provisioning and binding synchronously")
                .long("sync")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("curl")
                .help("Prints cURL command")
                .long("curl")
                .takes_value(false)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("provision")
                .about("Service Instance provisioning")
                .arg(
                    Arg::with_name("service")
                        .short("s")
                        .long("service")
                        .takes_value(true)
                        .required(true)
                        .help("service offering to use for provision"),
                )
                .arg(
                    Arg::with_name("plan")
                        .short("p")
                        .long("plan")
                        .takes_value(true)
                        .required(true)
                        .help("service plan to use for provision"),
                )
                .arg(
                    Arg::with_name("parameters")
                        .short("P")
                        .long("params")
                        .multiple(true)
                        .takes_value(true)
                        .help("parameters to provision service instances. ex: region=us-east-1 other=value"),
                )
                .arg(
                    Arg::with_name("context")
                        .short("C")
                        .long("context")
                        .multiple(true)
                        .takes_value(true)
                        .help("context to provision service instances. ex: account_id=123 other=value"),
                )
                .arg(
                    Arg::with_name("wait")
                        .short("w")
                        .long("wait")
                        .takes_value(false)
                        .help("wait service instance provisioning to finish"),
                ),
        )
        .subcommand(
            SubCommand::with_name("deprovision")
                .about("Service Instance deprovisioning")
                .arg(
                    Arg::with_name("instance")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .required(true)
                        .help("service instance id to deprovision"),
                ),
        )
        .subcommand(
            SubCommand::with_name("bind")
                .about("Service Binding request")
                .arg(
                    Arg::with_name("instance")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .help("instance ID or name to bind")
                        .required(true),
                )
                .arg(
                    Arg::with_name("binding")
                        .short("b")
                        .long("binding")
                        .takes_value(true)
                        .help("binding ID to fetch if bindings are fetchable")
                )
                .arg(
                    Arg::with_name("parameters")
                        .short("P")
                        .long("params")
                        .multiple(true)
                        .takes_value(true)
                        .help("parameters to provision service bindings. ex: param1=value1 param2=value2"),
                )
                .arg(
                    Arg::with_name("context")
                        .short("C")
                        .long("context")
                        .multiple(true)
                        .takes_value(true)
                        .help("context to provision service bindings. ex: account_id=123 other=value"),
                )
                .arg(
                    Arg::with_name("wait")
                        .short("w")
                        .long("wait")
                        .takes_value(false)
                        .help("wait service binding provisioning to finish")
                )
        )
        .subcommand(
            SubCommand::with_name("unbind")
                .about("Service Binding removal")
                .arg(
                    Arg::with_name("instance")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .help("instance ID or name to bind")
                        .required(true),
                )
                .arg(
                    Arg::with_name("binding")
                        .short("b")
                        .long("binding")
                        .takes_value(true)
                        .help("Binding ID to unbind")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("catalog")
                .about("Catalog request")
                .alias("cat"),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Fetch Service Instances information")
                .arg(
                    Arg::with_name("instance")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .help("instance ID to fetch information")
                        .required(true),
                )
        )
        .get_matches();

    let mut cfg = Configuration::new();
    cfg.user_agent = Some(cli::USER_AGENT.to_string());
    cfg.basic_auth = Some((
        matches.value_of("broker_user").unwrap().to_owned(),
        Some(matches.value_of("broker_pass").unwrap().to_owned()),
    ));
    cfg.base_path = matches.value_of("broker_url").unwrap().to_owned();

    let options = cli::Options {
        json_output: matches.is_present("json"),
        curl_output: matches.is_present("curl"),
        synchronous: matches.is_present("sync"),
    };

    let client = APIClient::new(cfg);

    match matches.subcommand_name() {
        Some("catalog") => cli::catalog(
            matches.subcommand_matches("catalog").unwrap(),
            client,
            options,
        ),
        Some("provision") => cli::provision(
            matches.subcommand_matches("provision").unwrap(),
            client,
            options,
        ),
        Some("deprovision") => cli::deprovision(
            matches.subcommand_matches("deprovision").unwrap(),
            client,
            options,
        ),
        Some("bind") => cli::bind(matches.subcommand_matches("bind").unwrap(), client, options),
        Some("unbind") => cli::unbind(
            matches.subcommand_matches("unbind").unwrap(),
            client,
            options,
        ),
        Some("info") => cli::info(matches.subcommand_matches("info").unwrap(), client, options),
        _ => Err(Box::from("unknown command")),
    }
}
