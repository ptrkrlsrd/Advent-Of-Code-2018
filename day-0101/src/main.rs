use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

extern crate regex;

struct Operation {
    prefix: String,
    number: i32,
}

fn subtract(a: i32, b:i32) -> i32 {
    return a - b;
}

fn add(a: i32, b:i32) -> i32 {
    return a + b;
}

fn calculate(op: Operation, sum: i32) -> i32 {
    match op.prefix.as_str() {
        "+" => {
            add(sum, op.number)
        },
        "-" => {
            subtract(sum, op.number)
        }
        _ => sum
    }
}

fn parse(caps: &regex::Captures) -> Operation {
    let prefix = caps.get(1).unwrap().as_str();
    let number_string = caps.get(2).unwrap().as_str();
    let number = number_string.parse::<i32>().unwrap();

    return Operation {
        prefix: String::from(prefix),
        number: number,
    }
}

fn main () {
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"([+-])(\d+)").unwrap();
    let mut sum = 0;
    let lines = reader.lines().map(| i | i.unwrap());
    
    lines.for_each(|l| {
        re.captures_iter(l.as_str()).for_each(|i| {
            let result = parse(&i);
            sum = calculate(result, sum);
        });
    });

    print!("{}", sum);
}
