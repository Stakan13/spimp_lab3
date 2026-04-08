use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::parser::parse_line;

pub struct CsvReader {
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
}

impl CsvReader {
    pub fn from_file(path: &str, delimiter: char) -> io::Result<Self> {
        let file = File::open(Path::new(path))?;
        let mut lines = io::BufReader::new(file).lines();

        let headers = match lines.next() {
            Some(line) => parse_line(&line?, delimiter),
            None => return Ok(Self { headers: vec![], records: vec![] }),
        };

        let records = lines
            .filter_map(|l| l.ok())
            .filter(|l| !l.trim().is_empty())
            .map(|l| parse_line(&l, delimiter))
            .collect();

        Ok(Self { headers, records })
    }

    pub fn print(&self) {
        println!("{}", self.headers.join(" | "));
        println!("{}", "-".repeat(40));
        for record in &self.records {
            println!("{}", record.join(" | "));
        }
    }
}
