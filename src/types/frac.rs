use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Frac {
    n: i64,
    d: i64,
}

impl Debug for Frac {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}
impl PartialEq for Frac {
    #[allow(dead_code)]
    fn eq(&self, other: &Self) -> bool {
        self.n * other.d == self.d * other.n
    }
}
impl Eq for Frac {}
impl Ord for Frac {
    #[allow(dead_code)]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.n * other.d).cmp(&(self.d * other.n))
    }
}
impl PartialOrd for Frac {
    #[allow(dead_code)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[allow(dead_code)]
impl Frac {
    fn zero() -> Self {
        Frac { n: 0, d: 1 }
    }
    fn one() -> Self {
        Frac { n: 1, d: 1 }
    }
    fn from(n: i64) -> Self {
        Frac { n, d: 1 }
    }
    fn approx(&mut self) {
        let g = Self::gcd(self.n, self.d);
        self.n /= g;
        self.d /= g;
    }
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
#[allow(dead_code)]
impl Add for Frac {
    type Output = Frac;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Frac {
            n: self.n * rhs.d + self.d * rhs.n,
            d: self.d * rhs.d,
        };
        ret.approx();
        ret
    }
}
#[allow(dead_code)]
impl Sub for Frac {
    type Output = Frac;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Frac {
            n: self.n * rhs.d - self.d * rhs.n,
            d: self.d * rhs.d,
        };
        ret.approx();
        ret
    }
}
#[allow(dead_code)]
impl Mul for Frac {
    type Output = Frac;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = Frac {
            n: self.n * rhs.n,
            d: self.d * rhs.d,
        };
        ret.approx();
        ret
    }
}
#[allow(dead_code)]
impl Div for Frac {
    type Output = Frac;

    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = Frac {
            n: self.n * rhs.d,
            d: self.d * rhs.n,
        };
        ret.approx();
        ret
    }
}
