use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main () {
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let lines = reader.lines().map(| i | i.unwrap());
    let changes = lines
        .map(|n| n
             .as_str()
             .parse::<i32>()
             .unwrap());
    
    let mut seen: HashSet<i32> = HashSet::new();

    for i in changes {
        sum += i;
        if seen.contains(&sum) {
            println!("{}", sum);
            break;
        } else {
            seen.insert(sum);
        }
    };
    
    println!("{}", sum);
}
