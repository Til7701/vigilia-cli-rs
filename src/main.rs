mod apis;
mod models;
mod index;
mod query;
mod vig_command;

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use clap::{crate_name, crate_version, Parser, Subcommand};
use tokio::runtime::Runtime;

fn main() {
    let args = VigArgs::parse();
    let config = Configuration::new();

    if args.version {
        show_version();
        return;
    }

    match args.command {
        Some(VigSubCommands::Index { paths }) => {
            if paths.is_empty() {
                index::dialog();
            } else {
                index::index_files(&config, paths);
            }
        }
        Some(VigSubCommands::Query { query }) => {
            if query.is_empty() {
                query::dialog();
            } else {
                let query = query.join(" ");
                query::query_files(&config, query);
            }
        }
        None => {
            vig_command::dialog();
        }
    }
}

#[derive(Parser)]
#[command(about, long_about = None, disable_version_flag = true)]
pub struct VigArgs {
    #[command(subcommand)]
    command: Option<VigSubCommands>,

    #[arg(short = 'V', long, help = "Print version")]
    version: bool,
}

#[derive(Subcommand)]
enum VigSubCommands {
    #[command(visible_alias = "i")]
    Index {
        #[arg(
            value_name = "PATHS",
            trailing_var_arg = true,
            allow_hyphen_values = true,
        )]
        paths: Vec<String>
    },
    #[command(visible_alias = "q")]
    Query {
        #[arg(
            value_name = "QUERY",
            trailing_var_arg = true,
            allow_hyphen_values = true,
        )]
        query: Vec<String>,
    },
}

fn show_version() {
    let config = Configuration::new();
    let config_clone = config.clone();
    let result = default_api::get_version(&config_clone);

    let cli_name = crate_name!();
    let cli_version = crate_version!();
    // get the api version from the user agent
    let user_agent = config.user_agent.unwrap();
    let cli_api_version = user_agent.split("/").nth(1).unwrap();

    println!("{cli_name} {cli_version}");
    println!("API version: {cli_api_version}");

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(core_version) => {
                println!("Service version: {}", core_version.service_version);
                println!("Service API version: {}", core_version.api_version);

                if core_version.api_version != cli_api_version {
                    println!("Warning: API version mismatch.");
                }
            }
            Err(e) => {
                println!("Could not get version of service: {}", e);
            }
        }
    });
}
