use num_bigint::BigInt as NumBigInt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigInt {
    value: NumBigInt,
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> BigInt {
        BigInt {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: BigInt) -> BigInt {
        BigInt {
            value: self.value - rhs.value,
        }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            value: self.value * rhs.value,
        }
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, rhs: BigInt) -> BigInt {
        BigInt {
            value: self.value / rhs.value,
        }
    }
}

impl Rem for BigInt {
    type Output = BigInt;

    fn rem(self, rhs: BigInt) -> BigInt {
        BigInt {
            value: self.value % rhs.value,
        }
    }
}

impl Neg for BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        BigInt { value: -self.value }
    }
}

impl From<i32> for BigInt {
    fn from(i: i32) -> Self {
        BigInt { value: i.into() }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

impl BigInt {
    pub fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {
        while b >= 1.into() {
            let r = a.clone() % b.clone();
            a = b;
            b = r;
        }

        a
    }

    pub fn lcm(a: BigInt, b: BigInt) -> BigInt {
        a.clone() * b.clone() / BigInt::gcd(a, b)
    }
}

#[cfg(test)]
mod tests {
    use crate::num::bigint::BigInt;

    #[test]
    fn test_addition() {
        assert_eq!(BigInt::from(2) + BigInt::from(2), BigInt::from(4));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(BigInt::gcd(2.into(), 4.into()), 2.into());
        assert_eq!(BigInt::gcd(4.into(), 2.into()), 2.into());
        assert_eq!(BigInt::gcd(37.into(), 43.into()), 1.into());
        assert_eq!(BigInt::gcd(43.into(), 37.into()), 1.into());
        assert_eq!(BigInt::gcd(215.into(), 86.into()), 43.into());
        assert_eq!(BigInt::gcd(86.into(), 215.into()), 43.into());
    }
}