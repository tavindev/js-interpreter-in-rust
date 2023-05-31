use js_interpreter_in_rust::parser::parser::Parser;

pub fn main() {
    let io = std::io::stdin();

    loop {
        let mut line = String::new();
        io.read_line(&mut line).unwrap();

        let parsed = Parser::new(line).parse();
        println!("{:?}", parsed);
    }
}
