use lalrpop_util::lalrpop_mod;
use num::Rational;
use std::convert::TryInto;

lalrpop_mod!(pub polynomial, "/parser/polynomial.rs");

#[derive(Debug)]
pub struct Term {
    coefficient: Rational,
    exponent: i32,
}

#[derive(Debug)]
pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<Rational>,
}

impl Term {
    fn new(coefficient: i32, exponent: i32) -> Term {
        Term {
            coefficient: Rational::new(coefficient.try_into().unwrap(), 1),
            exponent,
        }
    }
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Polynomial {
        let degree: usize = terms[0].exponent.try_into().unwrap();
        let mut coefficients = vec![Rational::new(0, 1); degree + 1];

        for t in terms.iter() {
            let exponent: usize = t.exponent.try_into().unwrap();
            coefficients[degree - exponent] = t.coefficient;
        }

        Polynomial {
            degree,
            coefficients,
        }
    }
}
