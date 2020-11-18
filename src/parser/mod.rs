use super::polynomial::Polynomial;
use lalrpop_util::lalrpop_mod;
use std::convert::TryInto;

lalrpop_mod!(pub polynomial, "/parser/polynomial.rs");

// XXX maybe make a command grammar and put polynomial in its own grammar
pub use polynomial::CommandParser;

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub polynomial: Polynomial,
}

#[derive(Debug)]
pub enum CommandType {
    Factor,
    Derivative,
    Integrate,
}

#[derive(Debug)]
pub struct Term {
    coefficient: i32,
    exponent: i32,
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
}

impl Term {
    fn new(coefficient: i32, exponent: i32) -> Term {
        Term {
            coefficient,
            exponent,
        }
    }
}

impl Polynomial {
    fn new(terms: Vec<Term>) -> Polynomial {
        let degree: usize = terms[0].exponent.try_into().unwrap();
        let mut coefficients = vec![0.0; degree + 1];

        for t in terms.iter() {
            let exponent: usize = t.exponent.try_into().unwrap();
            coefficients[exponent] = t.coefficient.into();
        }

        Polynomial {
            degree,
            coefficients,
        }
    }
}
