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
            let entries = parse_entries(&filename).unwrap();
            let n_rows = entries[0].len() as u32;

            let mut gamma_bits = Vec::<u32>::new();
            let mut epsilon_bits = Vec::<u32>::new();

            for col in entries {
                let sum: u32 = col.iter().sum();
                if sum > n_rows / 2 {
                    gamma_bits.push(1);
                    epsilon_bits.push(0);
                } else {
                    gamma_bits.push(0);
                    epsilon_bits.push(1);
                }
            }

            let gamma = vec_to_dec(&gamma_bits).unwrap();
            let epsilon = vec_to_dec(&epsilon_bits).unwrap();
            println!("Total: {}", gamma * epsilon);
        },
        1 | _ => {
        }
    }
}


fn vec_to_dec(array: &Vec::<u32>) -> Result<u32, &'static str> {
    let mut n = 0;
    for i in 0..array.len() {
        n += 2_u32.pow((array.len() - i - 1) as u32) * array[i];
    }

     Ok(n)
}


fn parse_entries(filename: &String) -> Result<Vec::<Vec::<u32>>, &'static str> {
    let file = File::open(filename).expect("Could not open file");
    let mut buf = io::BufReader::new(file);

    let mut first_line = String::new();
    buf.read_line(&mut first_line).expect("Could not read first line");

    let mut entries = Vec::<Vec::<u32>>::new();
    for c in first_line.trim_end().chars() {
        entries.push(vec![c.to_digit(10).unwrap()]);
    }

    for line in buf.lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            entries[i].push(c.to_digit(10).unwrap());
        }
    }

    Ok(entries)
}
