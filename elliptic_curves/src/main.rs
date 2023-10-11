use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;

use crate::field::FieldElement;

mod field;

fn main() {
    let x = FieldElement::new(BigInt::from(2), BigInt::from(9));
    let y = FieldElement::new(BigInt::from(7), BigInt::from(9));
    println!("{:?} + {:?} =", &x, &y);
    let z = x.clone() - y.clone();
    println!("{:?} + {:?} = {:?}", &x, &y, z);
    // ToDo - Tests
}

#[cfg(test)]
mod tests {
    use super::*;


}