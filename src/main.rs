mod reader;

fn main() {
    let content = reader::read_file("src/main.rs").unwrap();
    println!("{}", content);
}