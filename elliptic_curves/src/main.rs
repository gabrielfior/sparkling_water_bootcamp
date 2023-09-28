mod field;

use std::ops::{Add, Div, Mul, Sub};
use num_bigint::BigInt;

struct EllipticCurve {
    A: BigInt,
    B: BigInt
}

#[derive(Clone, Debug)]
struct Point {
    x: BigInt,
    y: BigInt
}

impl Point {
    fn equals(self, other: Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl EllipticCurve {
    fn add(self, p: Point, q:Point) -> Point {
        // addition
        let mut lambda = BigInt::from(45);
        let result = p.clone().equals(q.clone());
        println!("Result {}", result);

        if result{
            //lambda = ((3*&(p.x).mul(&p.x)).add(&self.A)).div(2*&p.y);
            lambda = p.x.clone().mul(3);
        }
        else {
            //lambda = (q.y.sub(p.y)).div( (&q.x.sub(&p.x)));
            lambda = (q.y.clone().sub(p.y.clone()));
        }
        println!("lambda {}", &lambda);
        let x3 = lambda.clone().mul(lambda.clone()).sub((p.x.clone()).add((q.x.clone())));

        let y3 = (lambda.clone().mul((p.x.clone()-x3.clone()))).sub(p.x.clone());
        println!("x3 {} y3 {}", &x3, &y3);
        Point{ x: x3, y: y3 }
    }
    fn mul(self, x: Point, y: Point) -> Point {

        Point {x: BigInt::from(2), y: BigInt::from(1)}
    }
}

fn main() {

    // ToDo - Tests
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_inverse_simple() {
        let p = Point { x: BigInt::from(1), y: BigInt::from(2) };
        let q = Point { x: BigInt::from(3), y: BigInt::from(4) };
        let ec = EllipticCurve {A: BigInt::from(-15),B:BigInt::from(18) };
        let r = ec.add(p, q);
        assert_eq!(r.x, BigInt::from(4));
    }
}