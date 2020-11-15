use super::polynomial::Polynomial;
use lalrpop_util::lalrpop_mod;
use std::convert::TryInto;

lalrpop_mod!(pub polynomial, "/parser/polynomial.rs");

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
