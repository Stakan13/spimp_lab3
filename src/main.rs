mod parser;
mod csv_reader;

use csv_reader::CsvReader;

fn main() {
    match CsvReader::from_file("data/sample.csv", ',') {
        Ok(csv) => csv.print(),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
