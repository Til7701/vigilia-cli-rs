use crate::apis::default_api;
use crate::config::api_config;
use crate::ui;
use crate::ui::format_error;
use spinners::{Spinner, Spinners};
use std::process::exit;
use tokio::runtime::Runtime;

pub fn dialog() {
    let selected = ui::select(
        "Which files do you want to index?",
        vec![
            "This directory".to_string(),
            "File or subdirectory".to_string(),
            "Exit".to_string(),
        ],
    );
    match selected {
        0 => index_files(vec![".".to_string()]),
        1 => {
            let mut path = ui::read_line("Enter the name of the file or subdirectory to index:");
            if path.is_empty() {
                println!("{}", format_error("Error: No file or subdirectory provided."));
                exit(1);
            }
            path = path.trim().parse().unwrap();
            if !path.starts_with("./") {
                path = format!("./{}", path);
            }
            index_files(vec![path]);
        }
        2 => exit(0),
        _ => unreachable!(),
    }
}

pub fn index_files(paths: Vec<String>) {
    let canonical_paths = paths
        .iter()
        .map(|path| {
            let path = std::path::Path::new(path);
            let canonical_path = path.canonicalize();
            match canonical_path {
                Ok(canonical_path) => canonical_path.to_str().unwrap().to_string(),
                Err(e) => {
                    println!("{}", format_error(&format!("Error: {}", e)));
                    exit(1);
                }
            }
        })
        .collect::<Vec<_>>();
    for path in &canonical_paths {
        println!("{}", path);
    }
    let mut spinner = Spinner::new(Spinners::Dots, "Indexing files".into());
    let config = api_config();
    let result = default_api::index_files(&config, canonical_paths);
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
