/// Parses a single CSV line into a vector of fields.
/// Handles quoted fields (e.g. "hello, world").
pub fn parse_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            c if c == delimiter && !in_quotes => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            c => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_line() {
        let result = parse_line("Alice,30,Engineer", ',');
        assert_eq!(result, vec!["Alice", "30", "Engineer"]);
    }

    #[test]
    fn test_quoted_field() {
        let result = parse_line("\"Smith, John\",25,Developer", ',');
        assert_eq!(result, vec!["Smith, John", "25", "Developer"]);
    }
}
