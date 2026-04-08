mod parser;
mod csv_reader;
mod filter;

use csv_reader::CsvReader;
use filter::print_filtered;

fn main() {
    let csv = match CsvReader::from_file("data/sample.csv", ',') {
        Ok(csv) => csv,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("=== All records ===");
    csv.print();

    println!("\n=== Filter: occupation = Developer ===");
    print_filtered(&csv, "occupation", "Developer");

    println!("\n=== Filter: age = 30 ===");
    print_filtered(&csv, "age", "30");
}
