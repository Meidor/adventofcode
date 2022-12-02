use chrono::{self, Datelike};
use reqwest::header::COOKIE;
use std::{env, fs::File, io::Write, path::Path};
use tera::{Context, Tera};
use dotenv::dotenv;

pub type Result<T> = std::result::Result<T, FetchError>;

#[derive(Debug, Clone)]
pub enum FetchError {
    Cause(String),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FetchError::Cause(cause) => {
                write!(f, "Error fetching Advent of Code input: {}", cause)
            }
        }
    }
}

fn write_template(tera: &Tera, template_name: &str, path: &Path, context: &Context) {
    let template = tera.render(template_name, context).unwrap();
    let mut output = File::create(path).unwrap();
    output.write_all(template.as_bytes()).unwrap();
}

fn main_template(tera: &Tera, days: &[String]) {
    let mut context = Context::new();
    context.insert("days", &days);
    write_template(tera, "main.rs.tera", Path::new("src/main.rs"), &context);
}

fn cargo_template(tera: &Tera, days: &[String]) {
    let mut context = Context::new();
    context.insert("days", &days);
    write_template(tera, "Cargo.toml.tera", Path::new("Cargo.toml"), &context);
}

fn mod_template(tera: &Tera, days: &[String]) {
    let mut context = Context::new();
    context.insert("days", &days);
    write_template(
        tera,
        "mod.rs.tera",
        Path::new("src/solutions/mod.rs"),
        &context,
    );
}

fn fetch_input(day: u32, year: i32, session: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input = reqwest::blocking::Client::new()
        .get(&url)
        .header(COOKIE, format!("session={}", session))
        .send();
    if input.is_err() {
        return Err(FetchError::Cause("Error sending GET request".to_string()));
    }

    let input = input.unwrap().text();

    if input.is_err() {
        return Err(FetchError::Cause(
            "Error converting input to string".to_string(),
        ));
    }

    let input = input.unwrap();

    if input.contains("Service Unavailable") {
        return Err(FetchError::Cause("Advent of Code is dead".to_string()));
    }

    if input.contains("Please don't repeatedly request") || input.contains("Not Found") {
        return Err(FetchError::Cause(format!(
            "Puzzle for day {} is not live yet",
            day
        )));
    }

    if input.contains("log in") {
        return Err(FetchError::Cause("Session cookie is invalid".to_string()));
    }

    if input.contains("Internal Server Error") {
        return Err(FetchError::Cause(
            "Internal Server Error, invalid session cookie perhaps?".to_string(),
        ));
    }

    Ok(input)
}

fn input_template(day: u32, year: i32, session: &str) {
    let ip = format!("inputs/day{:02}.txt", day);
    let input_path = Path::new(&ip);
    if !input_path.exists() {
        let input = fetch_input(day, year, session).unwrap();
        std::fs::write(input_path, input).unwrap();
    }
}

fn solution_template(tera: &Tera, day: &str, context: &Context) {
    let filename = format!("{}.rs", day);
    let day_file = tera.render("solution.rs.tera", context).unwrap();
    let dp = format!("src/solutions/{}", filename);
    let day_path = Path::new(&dp);
    if !day_path.exists() {
        let mut day_output = File::create(day_path).unwrap();
        day_output.write_all(day_file.as_bytes()).unwrap();
    }
}

fn benchmark_template(tera: &Tera, day: &str, context: &Context) {
    let filename = format!("{}.rs", day);
    let bench_file = tera.render("benchmark.rs.tera", context).unwrap();
    let bp = format!("benches/{}", filename);
    let bench_path = Path::new(&bp);
    if !bench_path.exists() {
        let mut bench_output = File::create(bench_path).unwrap();
        bench_output.write_all(bench_file.as_bytes()).unwrap();
    }
}

fn main() {
    dotenv().expect(".env vile not found");
    let session = env::var("AOC_SESSION").unwrap();
    let args: Vec<String> = env::args().skip(1).collect();
    let mut max_days: u32 = 1;
    let current_date = chrono::Local::now().date();
    let year: i32 = current_date.year();
    if args.is_empty() {
        let month = current_date.month();
        let day = current_date.day();
        if month == 12 {
            max_days = day;
        }
    } else {
        max_days = args[0].parse().unwrap();
    }

    let mut days_str = Vec::<String>::with_capacity(max_days as usize);
    let mut days = Vec::<u32>::with_capacity(max_days as usize);
    for day in 1..=max_days {
        days_str.push(format!("day{:02}", day));
        days.push(day);
    }

    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };


    main_template(&tera, &days_str);
    cargo_template(&tera, &days_str);
    mod_template(&tera, &days_str);
    for day in days {
        let d = format!("day{:02}", day);
        let mut context = Context::new();
        context.insert("day", &d);
        benchmark_template(&tera, &d, &context);
        solution_template(&tera, &d, &context);
        input_template(day, year, &session);
    }
}
