use std::ops::{Add, Sub, Mul};
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> FieldElement {
        if num.gt(&prime) {
            panic!("The number should be lesser than the prime");
        } else {
            FieldElement { num, prime }
        }
    }

    pub fn double(self: Self) -> FieldElement {
        let num = self.num.mul(2);
        FieldElement { num, prime: self.prime }
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self: Self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Only numbers in the same field can be added");
        } else {
            let num = (self.num + other.num) % &self.prime;
            FieldElement { num, prime: self.prime }
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self: Self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Only numbers in the same field can be subtracted");
        } else {
            if self.num > other.num {
                FieldElement { num: self.num - other.num, prime: self.prime }
            } else {
                FieldElement { num: self.num - other.num + &self.prime, prime: self.prime }
            }
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime.ne(&other.prime) {
            panic!("Cannot multiply two numbesr in different fields");
        }
        let num = &self.num.mul(&other.num) % &self.prime;
        FieldElement { num, prime: self.prime.clone() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_field_elements() {
        let x = FieldElement::new(BigInt::from(2), BigInt::from(9));
        let y = FieldElement::new(BigInt::from(1), BigInt::from(9));
        let z = x.clone() + y.clone();
        assert_eq!(z.num, BigInt::from(3));
    }

    #[test]
    fn test_double_field_elements() {
        let x = FieldElement::new(BigInt::from(2), BigInt::from(9));
        let z = x.double();
        assert_eq!(z.num, BigInt::from(4));
    }

    #[test]
    fn test_mul_field_elements() {
        let x = FieldElement::new(BigInt::from(2), BigInt::from(9));
        let y = FieldElement::new(BigInt::from(5), BigInt::from(9));
        let z = x.clone() * y.clone();
        assert_eq!(z.num, BigInt::from(1));
    }
}


