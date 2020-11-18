use num::complex::ComplexDistribution;
use num::rational::{Ratio, Rational32};
use num::{Complex, One, Signed, Zero};
use rand::distributions::uniform::Uniform;
use rand::Rng;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<f64>,
}

type C64 = Complex<f64>;

// XXX move this into an aberth module
fn offset_number_sum(curr_approximation: &Vec<C64>, k: usize) -> C64 {
    let z_k = curr_approximation[k];

    curr_approximation
        .iter()
        .enumerate()
        .fold(C64::zero(), |accum, (j, z_j)| {
            if j != k {
                accum + (C64::one() / (z_k - z_j))
            } else {
                accum
            }
        })
}

impl Polynomial {
    fn eval_at(&self, z: C64) -> C64 {
        let mut exp = 0;
        let mut result = C64::zero();

        while exp <= self.degree {
            result += self.coefficients[exp] * z.powi(exp as i32);
            exp += 1;
        }

        result
    }

    fn deriv_at(&self, z: C64) -> C64 {
        let mut exp = 1;
        let mut result = C64::zero();

        while exp <= self.degree {
            let coefficient: f64 = self.coefficients[exp] * (exp as f64);
            result += coefficient * z.powi(exp as i32 - 1);
            exp += 1;
        }

        result
    }

    // XXX move this into an aberth module
    fn offset_number(&self, curr_approximation: &Vec<C64>, k: usize) -> C64 {
        let z_k = curr_approximation[k];
        let t1 = self.eval_at(z_k) / self.deriv_at(z_k);

        t1 / (C64::one() - (t1 * offset_number_sum(curr_approximation, k)))
    }

    // XXX factor out an aberth function that takes closures for f(z) and f'(z)
    /// Return the factors of this polynomial. An approximation is first generated using the
    /// aberth method, and an attempt is made to turn that into an exact rational solution.
    fn factors(&self) -> Vec<Rational32> {
        let distribution =
            ComplexDistribution::new(Uniform::new(-10.0, 10.0), Uniform::new(-10.0, 10.0));
        let rng = rand::thread_rng();
        let mut curr_approximation = rng.sample_iter(&distribution).take(self.degree).collect();

        for _ in 0..100 {
            for j in 0..self.degree {
                let offset = self.offset_number(&curr_approximation, j);
                curr_approximation[j] -= offset;
            }
        }

        curr_approximation
            .into_iter()
            .map(|root| Ratio::approximate_float(root.re).unwrap())
            .collect()
    }

    pub fn derivative(&self) -> Self {
        self.clone()
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

    pub fn pretty(&self) -> String {
        let mut pretty = String::new();

        let mut iterator = self
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, x)| **x != 0.0);
        // XXX this assumes ahead of time that there are two or more terms
        // XXX fix
        let _last_term = iterator.next().unwrap();
        let mut iterator = iterator.rev();

        let first_term = iterator.next().unwrap();
        if first_term.1.is_negative() {
            std::write!(&mut pretty, "-").unwrap();
        }
        if *first_term.1 != 1.0 {
            std::fmt::write(&mut pretty, format_args!("{}", first_term.1)).unwrap();
        }
        if first_term.0 == 1 {
            std::write!(&mut pretty, "x").unwrap();
        } else if first_term.0 > 1 {
            std::fmt::write(&mut pretty, format_args!("x^{}", first_term.0)).unwrap();
        }

        for (_exp, _coef) in iterator {}

        pretty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_polynomial() -> Polynomial {
        Polynomial {
            degree: 2,
            coefficients: vec![2.0, 3.0, 1.0],
        }
    }

    #[test]
    fn test_eval_at() {
        let p = get_test_polynomial();

        assert_eq!(p.eval_at(C64::new(1.0, 0.0)), C64::new(6.0, 0.0));
        assert_eq!(p.eval_at(C64::new(0.0, 0.0)), C64::new(2.0, 0.0));
        assert_eq!(p.eval_at(C64::new(-1.0, 0.0)), C64::new(0.0, 0.0));
        assert_eq!(p.eval_at(C64::new(-2.0, 0.0)), C64::new(0.0, 0.0));
    }

    #[test]
    fn test_deriv_at() {
        let p = get_test_polynomial();

        assert_eq!(p.deriv_at(C64::new(1.0, 0.0)), C64::new(5.0, 0.0));
        assert_eq!(p.deriv_at(C64::new(2.0, 0.0)), C64::new(7.0, 0.0));
        assert_eq!(p.deriv_at(C64::new(3.0, 0.0)), C64::new(9.0, 0.0));
    }
}
