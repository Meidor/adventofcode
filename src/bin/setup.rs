use chrono::{self, Datelike};
use std::{env, fs::File, io::Write, path::Path};
use tera::{Context, Tera};

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

fn input_template(day: &str) {
    let ip = format!("inputs/{}.txt", day);
    let input_path = Path::new(&ip);
    if !input_path.exists() {
        File::create(input_path).unwrap();
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
    let args: Vec<String> = env::args().skip(1).collect();
    let mut max_days: usize = 1;

    if args.is_empty() {
        let current_date = chrono::Local::now().date();
        let month = current_date.month();
        let day = current_date.day();
        if month == 12 {
            max_days = day as usize;
        }
    } else {
        max_days = args[0].parse().unwrap();
    }

    let mut days = Vec::<String>::with_capacity(max_days);
    for day in 1..=max_days {
        days.push(format!("day{:02}", day));
    }

    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    main_template(&tera, &days);
    cargo_template(&tera, &days);
    mod_template(&tera, &days);
    for day in days {
        let mut context = Context::new();
        context.insert("day", &day);
        benchmark_template(&tera, &day, &context);
        solution_template(&tera, &day, &context);
        input_template(&day);
    }
}
