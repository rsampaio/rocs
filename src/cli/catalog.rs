use crate::cli::{generate_curl_command, Options};
use clap::ArgMatches;
use prettytable::{format, Table};
use rocl::apis::{catalog_api::catalog_get, configuration::Configuration};
use std::error::Error;

pub async fn catalog(
    _: &ArgMatches,
    config: Configuration,
    options: Options,
) -> Result<(), Box<dyn Error>> {
    let catalog = catalog_get(&config, crate::cli::DEFAULT_API_VERSION)
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
