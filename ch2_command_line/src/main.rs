mod ch2 {
    use clap::{App, Arg};
    use regex::Regex;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn p212() {
        let args = App::new("grep-lite")
            .version("0.1")
            .about("searches for patterns")
            .arg(
                Arg::with_name("pattern")
                    .help("The patternto search for")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

        let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverisly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

        for line in quote.lines() {
            match re.find(line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }

    pub fn p2131() {
        let f = File::open("readme.md").unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();

        loop {
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 {
                break
            }

            println!("{} ({} bytes long)", line, len);

            line.truncate(0);
        }
    }

    pub fn p2132() {
        let f = File::open("readme.md").unwrap();
        let mut reader = BufReader::new(f);

        for line_ in reader.lines() {
            let line = line_.unwrap();
            println!("{} ({} bytes long)", line, line.len());
        }
    }

    pub fn p2133() {
        let args = App::new("grep-lite")
            .version("0.1")
            .about("searchs for patterns")
            .arg(Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true))
            .get_matches();

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

        let input = args.value_of("input").unwrap();
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);

        for line_ in reader.lines() {
            let line = line_.unwrap();
            match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }
}

fn main() {
    // println!("____2.12 Supporting command-line arguments");
    // ch2::p212(); // cargo run -- picture

    // println!("____2.13 Reading from files");
    // ch2::p2131();

    // println!("____2.13 Reading from files");
    // ch2::p2132();

    println!("____2.13 Reading from files");
    ch2::p2133(); // cargo run -- incididunt readme.md
}
