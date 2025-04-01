use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::models;
use tokio::runtime::Runtime;

pub fn dialog() {
    println!("Querying files...");
}

pub fn query_files(config: &Configuration, query: String) {
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
