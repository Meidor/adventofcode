use chrono::{self, Datelike};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use reqwest::header::COOKIE;
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tera::{Context, Tera};

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

fn main_template(tera: &Tera, days: &[String], year: i32) {
    let mut context = Context::new();
    context.insert("days", &days);
    context.insert("year", &year);
    let path = format!("./aoc-{}/src/main.rs", year);
    write_template(tera, "main.rs.tera", Path::new(&path), &context);
}

fn cargo_template(days: &[String], year: i32) -> Result<()> {
    let path = format!("./aoc-{}/Cargo.toml", year); // Update with your file path

    // Read the existing content of Cargo.toml
    let content = fs::read_to_string(&path)?;

    // Define the markers
    let start_marker = "# BENCHMARK START";
    let end_marker = "# BENCHMARK END";

    // Find the positions of the markers
    let start_pos = content
        .find(start_marker)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Start marker not found"))?;
    let end_pos = content
        .find(end_marker)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "End marker not found"))?;

    // Split the content at the markers
    let mut new_content = String::new();
    new_content.push_str(&content[..start_pos + start_marker.len()]);

    // Append the new benchmark entries
    for day in days {
        new_content.push_str(&format!(
            "\n[[bench]]\nname = \"{}\"\nharness = false\n",
            day
        ));
    }

    new_content.push_str(&content[end_pos..]);

    // Write the updated content back to Cargo.toml
    let mut file = fs::File::create(path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn mod_template(tera: &Tera, days: &[String], year: i32) {
    let mut context = Context::new();
    context.insert("days", &days);
    let path = format!("./aoc-{}/src/solutions/mod.rs", year);
    write_template(tera, "mod.rs.tera", Path::new(&path), &context);
}

fn fetch_input(day: u32, year: i32, session: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input = reqwest::blocking::Client::new()
        .get(url)
        .header(COOKIE, format!("session={}", session))
        .send()?
        .text()?;
    Ok(input)
}

fn input_template(day: u32, year: i32, session: &str) {
    let ip = format!("./aoc-{}/inputs/day{:02}.txt", year, day);
    let input_path = Path::new(&ip);
    if !input_path.exists() {
        let input = fetch_input(day, year, session).unwrap();
        std::fs::write(input_path, input).unwrap();
    }
}

fn solution_template(tera: &Tera, day: &str, year: i32, context: &Context) {
    let filename = format!("{}.rs", day);
    let day_file = tera.render("solution.rs.tera", context).unwrap();
    let dp = format!("./aoc-{}/src/solutions/{}", year, filename);
    let day_path = Path::new(&dp);
    if !day_path.exists() {
        let mut day_output = File::create(day_path).unwrap();
        day_output.write_all(day_file.as_bytes()).unwrap();
    }
}

fn benchmark_template(tera: &Tera, day: &str, year: i32, context: &mut Context) {
    let filename = format!("{}.rs", day);
    context.insert("year", &year);
    let bench_file = tera.render("benchmark.rs.tera", context).unwrap();
    let bp = format!("./aoc-{}/benches/{}", year, filename);
    let bench_path = Path::new(&bp);
    if !bench_path.exists() {
        let mut bench_output = File::create(bench_path).unwrap();
        bench_output.write_all(bench_file.as_bytes()).unwrap();
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().expect(".env file not found");
    let session = env::var("AOC_SESSION").unwrap();
    let args: Vec<String> = env::args().skip(1).collect();
    let mut max_days: u32 = 1;
    let current_date = chrono::Local::now().date();
    let mut year: i32 = current_date.year();
    if args.is_empty() {
        let month = current_date.month();
        let day = current_date.day();
        if month == 12 {
            max_days = day;
        }
    } else {
        year = args[0].parse().unwrap();
        max_days = args[1].parse().unwrap();
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

    main_template(&tera, &days_str, year);
    cargo_template(&days_str, year)?;
    mod_template(&tera, &days_str, year);
    for day in days {
        let d = format!("day{:02}", day);
        let mut context = Context::new();
        context.insert("day", &d);
        benchmark_template(&tera, &d, year, &mut context);
        solution_template(&tera, &d, year, &context);
        input_template(day, year, &session);
    }

    Ok(())
}
