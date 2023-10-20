use num_bigint::BigUint;

enum Point {
    Coor(BigUint, BigUint),
    Identity,
}

struct EllipticCurve {
    // y^2 = x^2 + a * x + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {    
    fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not in curve");
        assert!(self.is_on_curve(d), "Point is not in curve");

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

    pub fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coor(x, y) => {
                // y^2 = x^3 + a * x + b
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);
                y2 == FiniteField::add(&x3plusax, &self.b, &self.p)
            }
            Point::Identity => true,
        }
    }
}

struct FiniteField {}

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d mod p = r
        let r = c + d;
        return r.modpow(&BigUint::from(1u32), p);
    }

    fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d mod p = r
        let r = c * d;
        return r.modpow(&BigUint::from(1u32), p);
    }

    fn inv_add(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        p - c
    }

    fn inv_mult(c: &BigUint, p: &BigUint) -> BigUint {
        // this function uses Fermat's Little Theorem and thus it is only valid for a p prime
        // c^(-1) mod p = c^(p-2) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add_1() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::add(&c, &d, &p);

        assert_eq!(r, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(31u32);

        let r = FiniteField::add(&c, &d, &p);

        assert_eq!(r, BigUint::from(14u32));
    }

    #[test]
    fn test_mult_1() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::mult(&c, &d, &p);

        assert_eq!(r, BigUint::from(7u32));
    }

    #[test]
    fn test_mult_2() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(11u32);
        let p = BigUint::from(31u32);

        let r = FiniteField::mult(&c, &d, &p);

        assert_eq!(r, BigUint::from(13u32));
    }
    
    #[test]
    fn test_inv_add_1() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let r = FiniteField::inv_add(&c, &p);

        assert_eq!(r, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_add_2() {
        let c = BigUint::from(52u32);
        let p = BigUint::from(51u32);

        FiniteField::inv_add(&c, &p);
    }

    #[test]
    fn test_inv_add_identity() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let c_inv = FiniteField::inv_add(&c, &p);

        assert_eq!(c_inv, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&c, &c_inv, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_inv_mult_identity() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        let c_inv = FiniteField::inv_mult(&c, &p);

        // 4 * 3 mod 11 = 12 mod 11 = 1
        assert_eq!(c_inv, BigUint::from(3u32));
        assert_eq!(FiniteField::mult(&c, &c_inv, &p), BigUint::from(1u32));
    }
}