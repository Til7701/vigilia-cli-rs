use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use spinners::{Spinner, Spinners};
use std::process::exit;
use tokio::runtime::Runtime;

pub fn dialog() {
    println!("Indexing files...");
}

pub fn index_files(config: &Configuration, paths: Vec<String>) {
    let canonical_paths = paths
        .iter()
        .map(|path| {
            let path = std::path::Path::new(path);
            let canonical_path = path.canonicalize();
            match canonical_path {
                Ok(canonical_path) => canonical_path.to_str().unwrap().to_string(),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        })
        .collect::<Vec<_>>();
    for path in &canonical_paths {
        println!("{}", path);
    }
    let mut spinner = Spinner::new(Spinners::Dots, "Indexing files".into());
    let result = default_api::index_files(config, canonical_paths);
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(_) => {
                spinner.stop_with_message("Indexing completed successfully.".into());
            }
            Err(e) => {
                spinner.stop_with_message(format!("Error indexing files: {}", e));
                exit(1);
            }
        }
    });
}