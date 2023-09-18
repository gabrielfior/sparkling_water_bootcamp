use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

#[derive(Debug)]
pub struct Prime;

// Based on https://github.com/aniketpr01/naive-rsa/blob/master/src/prime.rs
impl Prime {
    //  Rabin-Miller primality test
    #[allow(dead_code)]
    pub fn is_probably_prime(p: BigUint) -> bool {
        // Step 1: The number p is already provided as an argument

        let one = BigUint::one();
        let two = 2.to_biguint().unwrap();

        if p < two {
            return false;
        }

        if p == two {
            return true;
        }

        // Step 2: Find d such that 2 * d + 1 = p
        let mut d = (p.clone() - &one).clone();
        let mut r = 0;

        while &d % &two == BigUint::zero() {
            d /= &two;
            r += 1;
        }

        // Step 3: Choose a witness number y (randomly between 2 and p-2)
        // This witnesses can give us prime range to be over quadrillion numbers
        // This strong primes are better than having rounds because rounds can
        // have up to 25% chances that witness numbers generate false positives.
        let witnesses = vec![
            BigUint::from(2u32),
            BigUint::from(13u32),
            BigUint::from(23u32),
            BigUint::from(1662803u32),
        ];

        'outer: for a in witnesses {
            let mut x = a.modpow(&d, &p);

            if x == one || x == (p.clone() - &one) {
                continue;
            }

            for _ in 0..(r - 1) {
                // Step 4: Find x^2 mod p
                x = x.modpow(&two, &p);
                if x == (p.clone() - &one) {
                    continue 'outer;
                }
            }
            // definitely composite
            return false;
        }
        // probably prime
        true
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::Prime;

    #[test]
    fn test_is_probably_prime() {
        let p = BigUint::from(747u32);
        assert_eq!(Prime::is_probably_prime(p), false);

        let p = BigUint::from(743u32);
        assert_eq!(Prime::is_probably_prime(p), true);
    }
}
