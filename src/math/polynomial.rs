use std::vec::Vec;

/**
 * Represents a polynomial of degree n
 * of the form: 
 * f(x) = c0 + c1*X + c2*Xˆ2 + ... + cn*X^n
 */
 #[derive(Debug)]
pub struct Polynomial{
	n: u32,
	c0: f64,
	coeff: Vec<f64>
}

impl Polynomial{
	pub fn build(n: u32, c0: f64, coeff: &[f64]) -> Polynomial{
		//TOOD: Check size n
		Polynomial {n, c0, coeff: coeff.to_vec()}
	}

	pub fn apply(&self, x: f64) -> f64{
		let mut y : f64 = 0.0;
		for i in 1..self.n + 1{
			y += &self.coeff[i as usize - 1] * (x.powf(i as f64));
		}
		self.c0 + y
	}	
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn poly_build_works() {
        let poly = Polynomial::build(3, 164.0, &[48.0, 52.0, -20.0]);
        let expected: f64 = 78644.0;
        let result = poly.apply(-15.0);
        assert_eq!(true, true);
    }

    #[test]
    fn apply_works_neg_values() {
        let poly = Polynomial::build(3, 164.0, &[48.0, 52.0, -20.0]);
        let expected: f64 = 78644.0;
        let result = poly.apply(-15.0);
        assert_eq!(result, expected);

        let poly = Polynomial::build(2, -36.24, &[-158.0, 52.0]);
        let expected: f64 = 14033.76;
        let result = poly.apply(-15.0);
        assert_eq!(result, expected);

        let expected: f64 = 51841963.76;
        let result = poly.apply(1000.0);
        assert_eq!(result, expected);
    }
}

