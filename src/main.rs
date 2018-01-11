use std::fs::File;
use std::io::{BufReader, Error};
use std::io::prelude::*;
use std::env;

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).unwrap_or("book.txt".to_owned());
    let fd = File::open(filename).expect("unable to open file");
    let mut reader = BufReader::new(fd);

    let mut counter = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            for _ in l.split_whitespace() {
                counter += 1;
            }
        }
    }
    println!("words: {}", counter);
}
