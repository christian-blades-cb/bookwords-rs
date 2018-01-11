extern crate reqwest;
extern crate clap;
extern crate num;

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("bookwords-rs")
        .version("0.1")
        .about("counts words and teaches some rust")
        .subcommand(
            SubCommand::with_name("file")
                .about("counts words in a file")
                .arg(
                    Arg::with_name("filename")
                        .help("path to file")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("http")
                .about("counts words from a url")
                .arg(
                    Arg::with_name("url")
                        .help(
                            "url to book location (http://www.gutenberg.org/files/2600/2600-0.txt)",
                        )
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("iters").about(
            "stupid iterator tricks",
        ))
        .get_matches();

    match matches.subcommand() {
        ("file", Some(sub_m)) => {
            let filename = sub_m.value_of("filename").expect("filename required");
            let wordcount = words_in_file(filename).expect("unable to read file");
            println!("{}: {} words", filename, wordcount);
        }
        ("http", Some(sub_m)) => {
            let url = sub_m.value_of("url").expect("url required");
            let wordcount = words_in_url(url).expect("unable to read from url");
            println!("{}: {} words", url, wordcount);
        }
        ("iters", _) => {
            stupid_iterator_tricks();
        }
        _ => println!("{}", matches.usage()),
    }

}

fn words_in_file(filename: &str) -> Result<usize, std::io::Error> {
    let fd = File::open(filename)?;
    let reader = BufReader::new(fd);

    let mut counter = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            for _ in l.split_whitespace() {
                counter += 1;
            }
        }
    }

    Ok(counter)
}

fn words_in_url(url: &str) -> Result<usize, std::io::Error> {
    let mut resp = reqwest::get(url).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "unsuccessful request")
    })?;
    if !resp.status().is_success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "unsuccessful request",
        ));
    }

    let mut buf = String::new();
    resp.read_to_string(&mut buf)?;

    let count = buf.split_whitespace().count();
    Ok(count)
}

enum NumberClass {
    Even,
    Odd,
    Zero,
}

impl fmt::Display for NumberClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &NumberClass::Even => write!(f, "Even"),
            &NumberClass::Odd => write!(f, "Odd"),
            &NumberClass::Zero => write!(f, "Zero"),
        }
    }
}

impl From<usize> for NumberClass {
    fn from(num: usize) -> Self {
        if num == 0 {
            return NumberClass::Zero;
        }

        if num % 2 == 0 {
            NumberClass::Even
        } else {
            NumberClass::Odd
        }
    }
}

fn stupid_iterator_tricks() {
    let evens = (1..).filter(|x| x % 2 == 0);
    let first_ten_evens: Vec<usize> = evens.take(10).collect();
    // let first_ten_evens = evens.take(10).collect::<Vec<usize>>();
    println!("first 10 {:?}", first_ten_evens);

    let evens = (1..).filter(|x| x % 2 == 0);
    println!(
        "21 to 30 {:?}",
        evens.skip(20).take(10).collect::<Vec<usize>>()
    );

    let nclass = NumberClass::from(13);
    println!("thirteen is {}", nclass);
}
