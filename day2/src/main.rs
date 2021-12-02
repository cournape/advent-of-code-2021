use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

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

            let mut pos: i32 = 0;
            let mut depth: i32 = 0;

            for entry in entries {
                match entry {
                    Instruction::Forward(value) => {
                        pos += value;
                    },
                    Instruction::Up(value) => {
                        depth -= value;
                    }
                    Instruction::Down(value) => {
                        depth += value;
                    }
                }
            }
            println!("Pos {}, Depth {} -> {}", pos, depth, pos * depth);
        },
        1 | _ => {
            panic!("not implemented");
        }
    }
}

fn parse_line(line: &String) -> Result<Instruction, &'static str> {
    let parts = line.split_whitespace();
    let parts: Vec<&str> = parts.collect();

    match parts.len() {
        2 => {
            let name = &parts[0];
            let value: i32 = parts[1].parse().unwrap();

            let instruction = match name {
                &"up" => Instruction::Up(value),
                &"down" => Instruction::Down(value),
                &"forward" => Instruction::Forward(value),
                _ => {
                    return Err("Unrecognized opcode");
                }
            };
            return Ok(instruction);
        },
        _ => Err("Could not parse line"),
    }
}

fn parse_entries(filename: &String) -> Result<Vec::<Instruction>, &'static str> {
    let file = File::open(filename).expect("Could not open file");
    let buf = io::BufReader::new(file);

    let mut entries = Vec::<Instruction>::new();
    for line in buf.lines() {
        let line = line.unwrap();
        entries.push(parse_line(&line).unwrap());

    }

    Ok(entries)
}
