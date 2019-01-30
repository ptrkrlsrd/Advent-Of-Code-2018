use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;
use chrono::*;

extern crate chrono;
extern crate regex;

#[derive(Debug)]
enum EventType {
    Asleep,
    Awake,
}

fn main() {
    let file = File::open("./data.txt").expect("could not read file");
    let reader = BufReader::new(file);
    let re = Regex::new(r"(?:\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\]) (Guard #(\d+) begins shift|falls asleep|wakes up)").unwrap();

    let lines = reader
        .lines()
        .map(|i| i.unwrap());

    let mut guard_nr = 0;
    let mut last_event = EventType::Asleep;
    let mut guards = HashMap::new();
        
    lines.for_each(|i| {
        let captures = re.captures_iter(i.as_str());
        for u in captures {
                let date_string = u.get(1).unwrap().as_str();
                let parsed_date = Utc.datetime_from_str(date_string, "%Y-%m-%d %H:%M").unwrap();
            
                match u.get(3) {
                    Some(value) => guard_nr = value.as_str().parse::<i32>().unwrap(),
                    _ => ()
                }
            
                match u.get(4) {
                    Some(value) => match &value.as_str(){
                        &"wakes up" => last_event = EventType::Awake,
                        &"falls asleep" => last_event = EventType::Asleep,
                        _ => panic!("No event")
                    },
                    _ => ()
                }
            
                guards.entry(guard_nr).or_insert(parsed_date);
            
                println!("{} {} {:?}", parsed_date, guard_nr, last_event);
            };
        });
    }
