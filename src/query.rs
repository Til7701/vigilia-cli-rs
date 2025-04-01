use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::models;
use spinners::{Spinner, Spinners};
use tokio::runtime::Runtime;

pub fn dialog() {
    println!("Querying files...");
}

pub fn query_files(config: &Configuration, query: String) {
    let mut spinner = Spinner::new(Spinners::Dots, "Querying files".into());
    let result = default_api::search_files(config, &*query, Option::from(0), Option::from(10));
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(results) => {
                spinner.stop();
                show_query_results(results);
            }
            Err(e) => {
                spinner.stop_with_message(format!("Error querying files: {}", e));
            }
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
