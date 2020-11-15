use std::io::Write;

mod parser;
mod polynomial;

fn main() {
    let mut buf = String::new();
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let parser = parser::polynomial::PolynomialParser::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        buf.clear();
        let read = stdin.read_line(&mut buf).unwrap();
        if read == 0 {
            println!("");
            break;
        }

        let polynomial = parser.parse(&buf).unwrap();
        println!("{}", polynomial.pretty_factored());
    }
}
