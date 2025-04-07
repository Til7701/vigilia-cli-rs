mod apis;
mod models;
mod index;
mod query;
mod vig_command;
mod ui;
mod config;

use clap::{Parser, Subcommand};

fn main() {
    let args = VigArgs::parse();

    match args.command {
        Some(VigSubCommands::Index { paths }) => {
            if paths.is_empty() {
                index::dialog();
            } else {
                index::index_files(paths);
            }
        }
        Some(VigSubCommands::Query { query }) => {
            if query.is_empty() {
                query::dialog();
            } else {
                let query = query.join(" ");
                query::query_files(query);
            }
        }
        None => {
            vig_command::dialog();
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct VigArgs {
    #[command(subcommand)]
    command: Option<VigSubCommands>,
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
