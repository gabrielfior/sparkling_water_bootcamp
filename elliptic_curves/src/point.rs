use std::ops::{Add, Div, Mul};

use num_bigint::BigInt;

use crate::field::FieldElement;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: Option<FieldElement>,
    y: FieldElement,
    a: BigInt,
    b: BigInt,
}


impl Add for Point {
    type Output = Point;
    fn add(self: Self, other: Self) -> Self::Output {
        if (self.a).ne(&other.a) || self.b.ne(&other.b) {
            panic!("Points {:?} and {:?} are not in the same curve", &self, &other);
        }
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }
        if self.x.eq(&other.x) && self.y.ne(&other.y) {
            return Point {
                x: None,
                y: FieldElement { num: Default::default(), prime: Default::default() },
                a: Default::default(),
                b: Default::default(),
            };
        }
        let x1 = self.x.clone().unwrap();
        let mut x3: BigInt;
        let mut y3: BigInt;
        if self.x.ne(&other.x) {
            // # s=(y2-y1)/(x2-x1)
            let s_num = (other.y - self.y.clone());
            let s_den = (other.x.clone().unwrap() - x1.clone());
            let s: BigInt = s_num.num.div(s_den.num);
            // # x3=s**2-x1-x2
            x3 = s.modpow(&BigInt::from(2), &x1.prime).min(x1.clone().num).min(other.x.clone().unwrap().num);
            // # y3=s*(x1-x3)-y1
            y3 = s.mul((x1.clone().num - x3.clone())) - self.y.clone().num;
        }

        // Remaining case: self == other
        // # Formula (x3,y3)=(x1,y1)+(x1,y1)
        // # s=(3*x1**2+a)/(2*y1)
        let s_num: BigInt = x1.num.modpow(&BigInt::from(2), &x1.prime.clone()) * 3 + self.a.clone();
        let s_den: BigInt = self.y.clone().num.mul(BigInt::from(2));
        let s: BigInt = s_num.div(s_den);
        // # x3=s**2-2*x1
        x3 = s.modpow(&BigInt::from(2), &x1.prime) - (self.x.unwrap().clone().num.mul(
            BigInt::from(2)));
        // # y3=s*(x1-x3)-y1
        y3 = s.mul((x1.num - x3.clone())) - self.y.clone().num;
        Point {
            x: Some(FieldElement { num: x3, prime: x1.prime.clone() }),
            y: FieldElement { num: y3, prime: x1.prime.clone() },
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        a: BigInt,
        b: BigInt,
        prime: BigInt,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                a: BigInt::from(1),
                b: BigInt::from(2),
                prime: BigInt::from(223),
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_add_points_from_diff_curves_panic() {
        let setup = Setup::new();
        let x = Point {
            x: Some(FieldElement::new(BigInt::from(1),
                                      BigInt::from(7))),
            y: FieldElement::new(BigInt::from(1),
                                 BigInt::from(7)),
            a: BigInt::from(123),
            b: setup.b.clone(),
        };
        let y = Point {
            x: Option::from(FieldElement::new(BigInt::from(1),
                                              BigInt::from(7))),
            y: FieldElement::new(BigInt::from(1),
                                 BigInt::from(7)),
            a: setup.a.clone(),
            b: setup.b.clone(),
        };
        x.clone() + y.clone();
    }

    #[test]
    fn test_add_points_1() {
        let setup = Setup::new();

        // a = Point(x=3, y=7, a=5, b=7)
        let x = Point {
            x: Option::from(FieldElement::new(BigInt::from(3),
                                              setup.prime.clone())),
            y: FieldElement::new(BigInt::from(7),
                                 setup.prime.clone()),
            a: setup.a.clone(),
            b: setup.b.clone(),
        };
        // b = Point(x=-1, y=-1, a=5, b=7)
        let y = Point {
            x: Option::from(FieldElement::new(BigInt::from(-1),
                                              setup.prime.clone())),
            y: FieldElement::new(BigInt::from(-1),
                                 setup.prime.clone()),
            a: setup.a.clone(),
            b: setup.b.clone(),
        };
        let z = x + y;
        // self.assertEqual(a + b, Point(x=2, y=-5, a=5, b=7))
        assert_eq!(z, Point {
            x: Option::from(FieldElement { num: BigInt::from(2), prime: setup.prime.clone() }),
            y: FieldElement { num: BigInt::from(-5), prime: setup.prime.clone() },
            a: setup.a.clone(),
            b: setup.b.clone(),
        })
    }

    #[test]
    fn test_add_point_with_inf() {
        let setup = Setup::new();
        // a = Point(x=3, y=7, a=5, b=7)
        let x = Point {
            x: None,
            y: FieldElement::new(BigInt::from(7),
                                 setup.prime.clone()),
            a: setup.a.clone(),
            b: setup.b.clone(),
        };
        // b = Point(x=-1, y=-1, a=5, b=7)
        let y = Point {
            x: Option::from(FieldElement::new(BigInt::from(-1),
                                              BigInt::from(7))),
            y: FieldElement::new(BigInt::from(-1),
                                 BigInt::from(7)),
            a: setup.a.clone(),
            b: setup.b.clone(),
        };
        let z = x + y.clone();
        // self.assertEqual(a + b, Point(x=2, y=-5, a=5, b=7))
        assert_eq!(z, y)
    }
}