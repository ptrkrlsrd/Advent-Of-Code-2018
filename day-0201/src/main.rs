use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main () {
    let file = File::open("./data.txt").expect("could not read file");
    let reader = BufReader::new(file);
    let mut twos = 0;
    let mut threes = 0;

    reader.lines().for_each(|i| {
        let mut seen = HashMap::new();
        i.unwrap()
        .chars()
        .for_each(|c| {
            *seen.entry(c).or_insert(0) += 1;
        });

        let mut counts = seen.keys()
            .map(|key| seen.get(key).unwrap())
            .filter(|&i| *i > 1 && *i < 4)
            .map(|f| *f as i32)
            .collect::<Vec<i32>>();

        counts.dedup();
        counts.iter().for_each(|i| {
            match &i {
                2 => twos += 1,
                3 => threes += 1,
                _ => ()
            }
        });
    });

    println!("{} * {} = {}", twos, threes, twos * threes);
}
