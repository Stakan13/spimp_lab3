use crate::csv_reader::CsvReader;

pub fn filter_by_column<'a>(
    csv: &'a CsvReader,
    column: &str,
    value: &str,
) -> Vec<&'a Vec<String>> {
    let col_index = match csv.headers.iter().position(|h| h == column) {
        Some(i) => i,
        None => {
            eprintln!("Column '{}' not found", column);
            return vec![];
        }
    };

    csv.records
        .iter()
        .filter(|record| {
            record.get(col_index).map_or(false, |v| v == value)
        })
        .collect()
}

pub fn print_filtered(csv: &CsvReader, column: &str, value: &str) {
    let results = filter_by_column(csv, column, value);
    println!("Filter: {} = \"{}\"", column, value);
    println!("{}", csv.headers.join(" | "));
    println!("{}", "-".repeat(40));
    if results.is_empty() {
        println!("No records found.");
    } else {
        for record in results {
            println!("{}", record.join(" | "));
        }
    }
}
