use crate::apis::default_api;
use crate::config::api_config;
use crate::ui::format_error;
use crate::{models, ui};
use spinners::{Spinner, Spinners};
use std::process::exit;
use tokio::runtime::Runtime;

pub fn dialog() {
    let query = ui::read_line("Enter the query:");
    if query.is_empty() {
        format_error("No query provided.");
        exit(1);
    }
    query_files(query);
}

pub fn query_files(query: String) {
    let mut spinner = Spinner::new(Spinners::Dots, "Querying files".into());
    let config = api_config();
    let result = default_api::search_files(&config, &*query, Option::from(0), Option::from(10));
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match result.await {
            Ok(results) => {
                spinner.stop_with_message("".into());
                print!("{}", termion::cursor::Up(1));
                show_query_results(results);
            }
            Err(e) => {
                spinner.stop_with_message(format_error(format!("Error querying files: {}", e).as_str()));
            }
        }
    });
}

fn show_query_results(results: Vec<models::SearchResult>) {
    if results.is_empty() {
        println!("{}", format_error("No results found."));
        return;
    }

    let min_score = results.iter()
        .map(|result| result.score.unwrap())
        .fold(f64::MAX, |a, b| a.min(b));
    let max_score = results.iter()
        .map(|result| result.score.unwrap())
        .fold(f64::MIN, |a, b| a.max(b));

    for result in results.iter().take(results.len() - 1) {
        show_query_result(result.to_owned(), min_score, max_score);
        let size = termion::terminal_size().unwrap();
        let width = size.0 as usize;
        // print separator line
        let separator = format!("{:-<width$}", "", width = width);
        println!("{separator}");
    }
    show_query_result(results.last().unwrap().to_owned(), min_score, max_score);
}

fn show_query_result(result: models::SearchResult, min_score: f64, max_score: f64) {
    let score_style = get_color(min_score, max_score, result.score.unwrap());
    let score = result.score.unwrap();
    let reset = termion::style::Reset;
    let path_style = format!("{}{}", termion::style::Bold, termion::style::Underline);
    let path = result.path.unwrap();
    let text = result.text.unwrap()
        .split("\n")
        .take(3)
        .collect::<Vec<_>>()
        .join("\n");
    println!("{score_style}{score}{reset}: {path_style}{path}{reset}");
    println!("{text}");
}

fn get_color(min: f64, max: f64, value: f64) -> String {
    let ratio = (value - min) / (max - min);
    let green = (255.0 * ratio) as u8;
    let red = (255.0 * (1.0 - ratio)) as u8;
    termion::color::Rgb(red, green, 0).fg_string()
}
