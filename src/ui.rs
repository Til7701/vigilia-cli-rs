use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn select(prompt: &str, options: Vec<String>) -> usize {
    let stdin = stdin();
    let terminal = stdout()
        .into_raw_mode()
        .unwrap();
    terminal.activate_raw_mode().unwrap();

    let cursor_hide = termion::cursor::Hide;
    let styled_prompt = format_dialog_header(prompt);
    println!("{styled_prompt}{cursor_hide}\r");
    let mut selected = 0;
    print_options(&options, selected);
    for c in stdin.keys() {
        match c.as_ref().unwrap() {
            termion::event::Key::Up => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            termion::event::Key::Down => {
                if selected < options.len() - 1 {
                    selected += 1;
                }
            }
            termion::event::Key::Char('\n') => break,
            _ => {}
        }
        reprint_options(&options, selected);
    }
    print!("{}", termion::cursor::Show);
    terminal.suspend_raw_mode().unwrap();
    selected
}

fn reprint_options(options: &[String], selected: usize) {
    let mut reset_string = String::new();
    for _ in options {
        reset_string.push_str(termion::clear::CurrentLine.to_string().as_str());
        reset_string.push_str("\r");
        reset_string.push_str(termion::cursor::Up(1).to_string().as_str());
    }
    print!("{reset_string}");

    print_options(options, selected);
}

fn print_options(options: &[String], selected: usize) {
    for (i, option) in options.iter().enumerate() {
        print_option(option, i == selected, i == options.len() - 1);
    }
}

fn print_option(option: &str, selected: bool, last: bool) {
    let selected_style = termion::color::Fg(termion::color::Magenta);
    let reset = termion::style::Reset;
    if selected {
        println!("{selected_style}> {option}{reset}\r");
    } else {
        println!("  {option}\r");
    }
}

pub fn read_line(prompt: &str) -> String {
    let styled_prompt = format_dialog_header(prompt);
    println!("{styled_prompt}");
    print!(" > ");
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}

pub fn format_error(error: &str) -> String {
    format!("{}{error}{}", termion::color::Fg(termion::color::Red), termion::style::Reset)
}

pub fn format_dialog_header(header: &str) -> String {
    format!("{}{header}{}", termion::style::Bold, termion::style::Reset)
}
