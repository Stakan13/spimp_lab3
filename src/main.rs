mod parser;

fn main() {
    let line = "Alice,30,Engineer";
    let fields = parser::parse_line(line, ',');
    println!("Parsed: {:?}", fields);
}
