mod parser;

fn main() {
    let command = parser::parser();

    println!("{:?}", command);
}
