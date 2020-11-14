use super::parser::Polynomial;
use num::{Complex, One, Rational, Signed, Zero};
use std::convert::TryInto;

type Num = Complex<Rational>;

impl Polynomial {
    fn eval_at(&self, x: Num) -> Num {
        let mut exp = self.degree;
        let mut result = Num::new(Rational::new(0, 1), Rational::new(0, 1));

        while exp > 0 {
            result += Num::new(self.coefficients[self.degree - exp], Rational::new(0, 1))
                * x.powi(exp.try_into().unwrap());
            exp -= 1;
        }

        result += self.coefficients[self.degree];

        result
    }

    fn deriv_at(&self, x: Num) -> Num {
        let mut exp = self.degree - 1;
        let mut result = Num::new(Rational::new(0, 1), Rational::new(0, 1));

        while exp > 0 {
            let mut next_exp: isize = exp.try_into().unwrap();
            next_exp += 1;

            result += Num::new(
                self.coefficients[self.degree - exp - 1],
                Rational::new(0, 1),
            ) * Num::new(Rational::new(next_exp, 1), Rational::new(0, 1))
                * x.powi(exp.try_into().unwrap());
            exp -= 1;
        }

        result += self.coefficients[self.degree - 1];

        result
    }

    fn offset_number(&self, curr_approximation: &Vec<Num>, k: usize) -> Num {
        let z_k = curr_approximation[k];
        let t1 = self.eval_at(z_k) / self.deriv_at(z_k);

        let mut t2 = Num::zero();
        for j in 0..curr_approximation.len() {
            if j != k {
                let z_j = curr_approximation[j];
                t2 += Num::one() / (z_k - z_j)
            }
        }

        t1 / ((Num::one() - t1) * t2)
    }

    fn factors(&self) -> Vec<Rational> {
        // XXX make initial approximation different
        let mut curr_approximation = vec![
            Num::new(Rational::new(1, 1), Rational::new(1, 1)),
            Num::new(Rational::new(2, 1), Rational::new(1, 1)),
        ];

        for _ in 0..10 {
            for j in 0..self.degree {
                let offset = self.offset_number(&curr_approximation, j);
                curr_approximation[j] += offset;
            }
        }

        println!("{:?}", curr_approximation);

        vec![Rational::one(), Rational::one() + 1]
    }

    pub fn pretty_factored(&self) -> String {
        let factors = self.factors();

        let mut pretty = String::new();

        for factor in factors.iter() {
            let sign = if factor.is_positive() { "-" } else { "+" };
            std::fmt::write(&mut pretty, format_args!("(x {} {})", sign, factor.abs())).unwrap();
        }

        pretty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_at() {
        let p = Polynomial {
            degree: 2,
            coefficients: vec![
                Rational::new(1, 1),
                Rational::new(3, 1),
                Rational::new(2, 1),
            ],
        };

        assert_eq!(
            p.eval_at(Num::new(Rational::new(1, 1), Rational::new(0, 1))),
            Num::new(Rational::new(6, 1), Rational::new(0, 1))
        );
    }

    #[test]
    fn test_deriv_at() {
        let p = Polynomial {
            degree: 2,
            coefficients: vec![
                Rational::new(1, 1),
                Rational::new(3, 1),
                Rational::new(2, 1),
            ],
        };

        assert_eq!(
            p.deriv_at(Num::new(Rational::new(1, 1), Rational::new(0, 1))),
            Num::new(Rational::new(5, 1), Rational::new(0, 1))
        );
    }
}
