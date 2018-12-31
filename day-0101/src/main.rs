use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

extern crate regex;


struct Line {
    prefix: String,
    number: i32,
}

fn main () {
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"([+-])(\d+)").unwrap();

    let mut sum = 0;

    for l in reader.lines() {
        let line = l.unwrap();
        for caps in re.captures_iter(line.as_str()) {
            let result = parse(&caps);

            match result.prefix.as_str() {
                "+" => {
                    sum += result.number;
                },
                "-" => {
                    sum -= result.number;
                }
                _ => print!("nope\n"),
            }
        }
    }

    print!("{}", sum);
}

fn parse(caps: &regex::Captures) -> Line {
    let prefix = caps.get(1).unwrap().as_str();
    let number_string = caps.get(2).unwrap().as_str();
    let number = number_string.parse::<i32>().unwrap();

    return Line {
        prefix: String::from(prefix),
        number: number,
    }
}