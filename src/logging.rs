use chrono::Local;
use colored::*;
use std::path::Path;

pub fn info(msg: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{} {} {}", time.dimmed(), "INFO".blue(), msg);
}

pub fn success(msg: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{} {} {}", time.dimmed(), "SUCCESS".green(), msg);
}

pub fn error(msg: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    eprintln!("{} {} {}", time.dimmed(), "ERROR".red(), msg);
}

pub fn warning(msg: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{} {} {}", time.dimmed(), "WARN".yellow(), msg);
}

pub fn status_change(path: &Path, change_type: &str, color: colored::Color) {
    let time = Local::now().format("%H:%M:%S").to_string();
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    let symbol = match change_type {
        "added" => "+",
        "modified" => "~",
        "renamed" => "→",
        "deleted" => "-",
        _ => " "
    };
    println!(
        "{} {} {} {}",
        time.dimmed(),
        symbol.color(color),
        change_type.color(color),
        filename
    );
}

pub fn git_operation(operation: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!(
        "{} {} {}",
        time.dimmed(),
        "GIT".cyan(),
        operation
    );
}

pub fn startup_message(path: &Path) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("\n{} {} Auto Git Sync", time.dimmed(), "STARTUP".bright_blue());
    println!("{} {} Monitoring directory: {}", 
        time.dimmed(),
        "STARTUP".bright_blue(),
        path.display().to_string().cyan()
    );
    println!("{} {} Press Ctrl+C to stop\n", 
        time.dimmed(),
        "STARTUP".bright_blue()
    );
}

pub fn init_message(msg: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{} {} {}", time.dimmed(), "INIT".magenta(), msg);
}
