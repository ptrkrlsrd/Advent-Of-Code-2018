use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main () {
    let file = File::open("./data.txt").unwrap();
    let sum = read(file);
    println!("{}", sum);
}

fn read(file: File) -> i32 {
    let reader = BufReader::new(file);
    let mut sum = 0;
    let lines = reader.lines().map(| i | i.unwrap());
    let changes = lines
        .map(|n| n
             .as_str()
             .parse::<i32>()
             .unwrap());
    
    let mut history = HashMap::new();

    for i in changes {
        sum += i;
        *history.entry(sum).or_insert(0) += 1;
    };

    for i in history.keys() {
        if history.get(i).unwrap() > &1 {
            println!("{}: {}", i, history.get(i).unwrap());
        }
    }

    return sum;
}
