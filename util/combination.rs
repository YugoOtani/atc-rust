struct Cmb {
    md: usize,
    fact: Vec<usize>,
}
impl Cmb {
    fn new(n: usize, md: usize) -> Self {
        let mut v = vec![0; n + 1];
        v[0] = 1;
        for i in 1..=n {
            v[i] = (v[i - 1] * i) % md;
        }
        Cmb { md, fact: v }
    }
    fn cmb(&self, n: usize, k: usize) -> usize {
        let md = self.md as i64;
        let kinv = Self::inv(self.fact[k] as i64, md);
        let nkinv = Self::inv(self.fact[n - k] as i64, md);
        let n = self.fact[n] as i64;
        ((((n * kinv) % md) * nkinv) % md) as usize
    }
    fn inv(a: i64, md: i64) -> i64 {
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
