use std::ops::{Add, Mul};

use num_bigint::{BigUint, RandBigInt};
use num_integer::{div_rem, Integer};
use num_traits::{CheckedSub, One, Zero};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::mulinv::FindInverse;
use crate::prime::Prime;

mod mulinv;
mod prime;

fn find_next_prime(n: BigUint) -> BigUint {
    let mut candidate = n;
    loop {
        if Prime::is_probably_prime(candidate.clone()) {
            return candidate;
        }
        candidate = candidate.add(BigUint::one());
    }
}

fn main() {
    // ToDo - Revisit this, split into classes, write tests
    //let mut rng = rand::thread_rng(1);
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    let p: BigUint = rng.gen_biguint(20);
    let mut p_prime = find_next_prime(p.clone());
    //p_prime = BigUint::from(769u64);
    let q: BigUint = rng.gen_biguint(20);
    let mut q_prime = find_next_prime(q.clone());
    //q_prime = BigUint::from(439u64);

    println!("p_prime {} q_prime {}", p_prime, q_prime);

    let n = &p_prime * &q_prime;
    println!("n {}", n);
    let phi = (p_prime - BigUint::one()) * (q_prime - BigUint::one());
    println!("phi {}", phi);
    let e = rng.gen_biguint_range(&BigUint::zero(), &phi);
    let mut e_prime = find_next_prime(BigUint::from(e.clone()));
    //e_prime = BigUint::from(28387u64);
    println!("e_prime {}", e_prime);
    //let mut d = FindInverse::find_inverse(&e_prime, &phi).unwrap();
    let mut d = FindInverse::find_inverse(&e_prime, &phi).unwrap();
    // for fixed values, d should be
    //d = BigUint::from(53781207401u64);
    println!("d {}", &d);
    let div_and_rem = div_rem(d.clone().mul(&e_prime), phi.clone());
    println!("remainder {}", div_and_rem.1);

    // Encrypting/decrypting
    let x = BigUint::from(123456789u64);
    let y = x.modpow(&e_prime, &n);
    println!("y {}", &y);
    let converted_x = y.modpow(&d, &n);
    println!("back to x {}", &converted_x);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encrypt_decrypt() {
        // ToDo - After classes set, write tests.
        assert_eq!(1, 1);
    }
}
