use clap::{Parser, Subcommand};
use std::process::exit;


fn main() {
    let args = VigArgs::parse();

    match args.command {
        Some(VigSubCommands::Index { paths }) => {
            for path in paths {
                println!("Indexing path: {}", path);
            }
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
    }
}

