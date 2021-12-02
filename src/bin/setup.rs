use std::{fs::File, io::Write, path::Path};

use tera::{Tera, Context};

fn main() {
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    let days =  ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"];
    context.insert("days", &days);
    let main_file = tera.render("main.rs.tera", &context).unwrap();
    let main_path = Path::new("src/main.rs");

    if !main_path.exists(){
        let mut main_output = File::create(main_path).unwrap();
        main_output.write_all(main_file.as_bytes()).unwrap();
    }
    
    for day in days {
        let mut context = Context::new();
        context.insert("number", &day);
        
        let filename = format!("day{}.rs", day);
        let bench_file = tera.render("benchmark.rs.tera", &context).unwrap();
        let bp = format!("benches/{}", filename);
        let bench_path = Path::new(&bp);
        if !bench_path.exists() {
            let mut bench_output = File::create(bench_path).unwrap();
            bench_output.write_all(bench_file.as_bytes()).unwrap();
        }
        
        let day_file = tera.render("solution.rs.tera", &context).unwrap();
        let dp = format!("src/solutions/{}", filename);
        let day_path = Path::new(&dp);
        if !day_path.exists() {
            let mut day_output = File::create(day_path).unwrap();
            day_output.write_all(day_file.as_bytes()).unwrap();
        }
        
        let ip = format!("inputs/day{}.txt", day);
        let input_path = Path::new(&ip);
        if !input_path.exists() {
            File::create(input_path).unwrap();
        }
    }
}