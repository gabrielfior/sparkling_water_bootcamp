#[derive(Clone, Debug, PartialEq)]
struct FieldElement {
    num: u128,
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
    fn add(self, other: Point) -> Point {
        self
    }

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.b == other.b && self.a == other.a
    }
}

#[cfg(test)]
mod tests {
    //use num_bigint::BigUint;
    use crate::field::{FieldElement, Point};

    #[test]
    fn test_field_elements_add() {
        let prime = 223;
        let a = FieldElement { num: 0, prime: prime };
        let b = FieldElement { num: 7, prime: prime };
        let p1 = Point { a: a.clone(), b:b.clone(), x: FieldElement { num: 170, prime }, y: FieldElement { num: 142, prime } };
        let p2 = Point { a:a.clone(), b:b.clone(), x: FieldElement { num: 60, prime }, y: FieldElement { num: 139, prime } };
        let sum1 = p1.add(p2);
        assert_eq!(sum1, Point {
            x: FieldElement { num: 220, prime },
            y: FieldElement { num: 181, prime },
            a: a.clone(),
            b: b.clone()
        });
    }
}