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
            panic!("Not implemented");
        }
    }
}

fn solve_main_problem(filename: &String) -> Result<usize, &'static str> {
    let file = File::open(filename).expect("Could not open file");
    let buf = io::BufReader::new(file);

    let mut entries = Vec::<i32>::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let entry: i32 = line.parse().unwrap();
        entries.push(entry);
    }

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
