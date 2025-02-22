use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Range, Sub};
type IntForMint = i64;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Mint {
    md: IntForMint,
    i: IntForMint,
}
#[allow(dead_code)]
impl Debug for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M({})", self.i)
    }
}
impl Add for Mint {
    type Output = Mint;

    fn add(self, rhs: Self) -> Self::Output {
        self.addi(rhs.i)
    }
}
impl Sub for Mint {
    type Output = Mint;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subi(rhs.i)
    }
}
impl Mul for Mint {
    type Output = Mint;

    fn mul(self, rhs: Self) -> Self::Output {
        self.muli(rhs.i)
    }
}
impl Div for Mint {
    type Output = Mint;

    fn div(self, rhs: Self) -> Self::Output {
        self.divi(rhs.i)
    }
}
#[allow(dead_code)]
impl Mint {
    fn frac(&self) -> String {
        self.frac_rng(2..10000)
    }
    fn frac_rng(&self, rng: Range<IntForMint>) -> String {
        for i in rng {
            let j = i * self.i % self.md;
            if j < i {
                return format!("{}/{}", j, i);
            }
        }
        format!("{}", self.i)
    }
    fn n(md: IntForMint, i: IntForMint) -> Self {
        Self { md, i }
    }
    fn zero(md: IntForMint) -> Self {
        Self { md, i: 0 }
    }
    fn one(md: IntForMint) -> Self {
        Self { md, i: 1 }
    }
    fn addi(self, i: IntForMint) -> Self {
        Self {
            i: (self.i + i) % self.md,
            md: self.md,
        }
    }
    fn subi(self, i: IntForMint) -> Self {
        Self {
            i: (self.i + 2 * self.md - i) % self.md,
            md: self.md,
        }
    }
    fn muli(self, i: IntForMint) -> Self {
        Self {
            i: (self.i * i) % self.md,
            md: self.md,
        }
    }
    fn divi(self, i: IntForMint) -> Self {
        let iinv = (Self::mod_inv(i as i64, self.md as i64) as IntForMint) % self.md;
        Self {
            i: (self.i * iinv) % self.md,
            md: self.md,
        }
    }
    fn mod_inv(a: i64, md: i64) -> i64 {
        let mut a = a;
        let mut b = md;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            (a, b) = (b, a);
            u -= t * v;
            (u, v) = (v, u);
        }
        u %= md;
        if u < 0 {
            u += md;
        }
        u
    }
}
