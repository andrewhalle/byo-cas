use std::io::Write;

mod parser;
mod polynomial;

use parser::CommandType;

fn main() {
    let mut buf = String::new();
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let parser = parser::CommandParser::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        buf.clear();
        let read = stdin.read_line(&mut buf).unwrap();
        if read == 0 {
            println!("");
            break;
        }

        let command = parser.parse(&buf).unwrap();
        match command.command_type {
            CommandType::Factor => println!("{}", command.polynomial.pretty_factored()),
            CommandType::Derivative => println!("{}", command.polynomial.derivative().pretty()),
            CommandType::Integrate => unimplemented!(),
        }
    }
}
