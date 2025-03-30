mod apis;
mod models;

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use clap::{Parser, Subcommand};
use std::process::exit;
use tokio::runtime::Runtime;

fn main() {
    let args = VigArgs::parse();
    let config = Configuration::new();

    match args.command {
        Some(VigSubCommands::Index { paths }) => {
            index_sub_command(&config, paths);
        }
        Some(VigSubCommands::Query { query }) => {
            query_sub_command(&config, query);
        }
        None => {
            println!("No command provided");
            exit(1);
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct VigArgs {
    #[command(subcommand)]
    command: Option<VigSubCommands>,
}

#[derive(Subcommand)]
enum VigSubCommands {
    Index {
        #[arg(short, long, value_name = "PATHS")]
        paths: Vec<String>
    },
    Query {
        #[arg(short, long, value_name = "QUERY")]
        query: String,
    },
}

fn index_sub_command(config: &Configuration, paths: Vec<String>) {
    let result = default_api::index_files(config, paths);
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(_) => println!("Indexing completed successfully."),
            Err(e) => eprintln!("Error indexing files: {}", e),
        }
    });
}


fn query_sub_command(config: &Configuration, query: String) {
    let mut query = query;
    let result = default_api::search_files(config, &*query, Option::from(0), Option::from(10));
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(results) => show_query_results(results),
            Err(e) => eprintln!("Error searching files: {}", e),
        }
    });
}

fn show_query_results(results: Vec<models::SearchResult>) {
    for result in results {
        println!("{}: {}", result.score.unwrap(), result.path.unwrap());
        println!("{}", result.text.unwrap());
        println!("-----------------------------");
    }
}
