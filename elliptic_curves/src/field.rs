use std::ops::{Div, Mul};
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
struct FieldElement {
    num: Option<BigInt>,
    prime: u128
}

impl FieldElement {
    fn add(self) -> u8 {
        1
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {

    fn build_point_at_infinity(self) -> Point{
        println!("Returning point at infinity");
        return Point {x: FieldElement {num: None, prime: self.x.prime},
            y: FieldElement {num: None, prime: self.x.prime},
            a: self.a, b: self.b }
    }

    fn add(self, other: Point) -> Point {

        // not same curve
        if self.a.num != other.a.num || self.b.num != other.b.num {
            panic!("Points on different curves");
        }
        // ToDo - if self.x is None
        if self.x.num == None {
            return other;
        }
        if other.x.num == None {
            return self;
        }
        // ToDo - if other.x is None
        if self.x.num.clone().unwrap().eq(&other.x.num.clone().unwrap()) &&
            !self.y.num.clone().unwrap().eq(&other.y.num.clone().unwrap()){
            // infinity
            return self.build_point_at_infinity()
        }
        // # Case 2: self.x ≠ other.x
        // # Formula (x3,y3)==(x1,y1)+(x2,y2)
        // # s=(y2-y1)/(x2-x1)
        // # x3=s**2-x1-x2
        // # y3=s*(x1-x3)-y1
        let s = (other.y.num.clone().unwrap().min(self.y.num.clone().unwrap())).div(
            other.x.num.clone().unwrap().min(self.x.num.clone().unwrap()));
        let x3 = s.modpow(&BigInt::from(2), &BigInt::from(self.x.prime)).min(
            self.x.num.clone().unwrap()).min(other.x.num.clone().unwrap());
        let y3 = s.mul(self.x.num.clone().unwrap().min(x3.clone())).min(self.y.num.clone().unwrap());
        return Point {
            x: FieldElement {num: Some(x3.clone()), prime: self.x.prime},
            y: FieldElement {num: Some(y3), prime: self.x.prime},
            a: self.a,
            b: self.b,
        };
    }

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.b == other.b && self.a == other.a
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    //use num_bigint::BigUint;
    use crate::field::{FieldElement, Point};

    #[test]
    fn test_field_elements_add_result_infinity(){
        let prime = 223;
        let a = FieldElement { num: Some(BigInt::from(0)), prime: prime };
        let b = FieldElement { num: Some(BigInt::from(7)), prime: prime };
        let p1 = Point { a: a.clone(), b:b.clone(),
            x: FieldElement { num: Some(BigInt::from(10)), prime },
            y: FieldElement { num: Some(BigInt::from(142)), prime } };
        let p2 = Point { a:a.clone(), b:b.clone(), x: FieldElement { num: Some(BigInt::from(10)), prime },
            y: FieldElement { num: Some(BigInt::from(139)), prime } };
        let sum1 = p1.add(p2);
        assert_eq!(sum1, Point {
            x: FieldElement { num: None, prime },
            y: FieldElement { num: None, prime },
            a: a.clone(),
            b: b.clone()
        });
    }

    fn test_field_elements_add1() {
        let prime = 223;
        let a = FieldElement { num: Some(BigInt::from(0)), prime: prime };
        let b = FieldElement { num: Some(BigInt::from(7)), prime: prime };
        let p1 = Point { a: a.clone(), b:b.clone(),
            x: FieldElement { num: Some(BigInt::from(170)), prime },
            y: FieldElement { num: Some(BigInt::from(142)), prime } };
        let p2 = Point { a:a.clone(), b:b.clone(), x: FieldElement { num: Some(BigInt::from(60)), prime },
            y: FieldElement { num: Some(BigInt::from(139)), prime } };
        let sum1 = p1.add(p2);
        assert_eq!(sum1, Point {
            x: FieldElement { num: Some(BigInt::from(220)), prime },
            y: FieldElement { num: Some(BigInt::from(181)), prime },
            a: a.clone(),
            b: b.clone()
        });
    }


}