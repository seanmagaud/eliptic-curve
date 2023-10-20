use num_bigint::BigUint;

struct Point {
    x: BigUint,
    y: BigUint,
}

struct EllipticCurve {
    // y^2 = x^2 + a * x + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn add(c: &Point, d: &Point) -> Point {
        todo!("Implement EllipticCurve::add")
    }

    fn double(c: &Point) -> Point {
        todo!("Implement EllipticCurve::double")
    }

    fn scalar_mul(c: &Point, d: &BigUint) -> Point {
        // addition/doubling  algorithm
        // B = d * A
        todo!("Implement EllipticCurve::scalar_mul")
    }
}

struct FiniteField {}

impl FiniteField{
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d mod p = r
        todo!("Implement FiniteField::add")
    }

    fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d mod p = r
        todo!("Implement FiniteField::mult")
    }

    fn inv_add(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        todo!("Implement FiniteField::inv_add")
    }

    fn inv_mult(c: &BigUint, p: &BigUint) -> BigUint {
        // c^(-1) mod p
        todo!("Implement FiniteField::inv_mult")
    }
}