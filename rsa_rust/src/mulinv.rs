use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::One;
use num_traits::Zero;
use num_traits::{one, zero};
use std::ops::{Add, Div, Mul};

#[derive(Debug)]
pub struct FindInverse;

// Based on https://github.com/aniketpr01/naive-rsa/blob/master/src/mulinv.rs
#[allow(dead_code)]
impl FindInverse {
    pub fn find_inverse(e: &BigUint, phi: &BigUint) -> Option<BigUint> {
        let mut r1 = BigInt::from(phi.clone());
        let mut r2 = BigInt::from(e.clone());
        let mut t1 = BigInt::zero();
        let mut t2 = BigInt::one();

        // Loop until r becomes zero
        while !r2.is_zero() {
            let quotient = &r1 / &r2;

            let temp_r = r2.clone();
            r2 = &r1 % &r2;
            r1 = temp_r;

            let temp_t = t2.clone();
            t2 = &t1 - (&quotient * &t2);
            t1 = temp_t;
        }

        if r1 != BigInt::one() {
            return None; // e and phi are not coprime
        }

        if t1 < BigInt::zero() {
            return Some((t1 + BigInt::from(phi.clone())).to_biguint().unwrap());
        }
        Some(t1.to_biguint().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_inverse_simple(){
        let e = BigUint::from(6u32);
        let phi = BigUint::from(11u32); // Assuming n is prime for this example
        let inverse = FindInverse::find_inverse(&e, &phi).unwrap();
        assert_eq!(inverse, BigUint::from(2u32));
    }
    #[test]
    fn test_find_inverse() {

        let e = BigUint::from(65537u64);
        let phi = BigUint::from(401704021805227919u64); // Assuming n is prime for this example
        let inverse = FindInverse::find_inverse(&e, &phi).unwrap();
        assert_eq!(inverse, BigUint::from(186739013508675615u64));

    }
}
