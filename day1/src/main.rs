use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("day 1")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("part-two")
            .short("2")
            .long("part-two")
            .help("If set, solves part 2"))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap().to_string();
    println!("Using input file: {}", filename);

    match matches.occurrences_of("part-two") {
        0 => {
            let n_increases = solve_main_problem(&filename).unwrap();
            println!("{} increases", n_increases);
        },
        1 | _ => {
            let n_increases = solve_secondary_problem(&filename).unwrap();
            println!("{} increases", n_increases);
        }
    }
}

fn parse_entries(filename: &String) -> Result<Vec::<i32>, &'static str> {
    let file = File::open(filename).expect("Could not open file");
    let buf = io::BufReader::new(file);

    let mut entries = Vec::<i32>::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let entry: i32 = line.parse().unwrap();
        entries.push(entry);
    }

    Ok(entries)
}

fn find_n_increases(entries: Vec<i32>) -> Result<usize, &'static str> {
    let mut acc: usize = 0;
    if entries.len() >= 1 {
        for i in 1..entries.len() {
            let diff: i64 = (entries[i] - entries[i-1]).into();
            if diff > 0 {
                acc += 1;
            }
        }
    }

    Ok(acc)
}

fn solve_main_problem(filename: &String) -> Result<usize, &'static str> {
    let entries = parse_entries(filename).unwrap();
    Ok(find_n_increases(entries).unwrap())
}


fn solve_secondary_problem(filename: &String) -> Result<usize, &'static str> {
    let entries = parse_entries(filename).unwrap();

    let mut windows = Vec::<i32>::new();

    if entries.len() >= 3 {
        for i in 1..(entries.len() - 1) {
            let window: i32 = entries[i+1] + entries[i] + entries[i-1];
            windows.push(window);
        }
    }

    Ok(find_n_increases(windows).unwrap())
}
