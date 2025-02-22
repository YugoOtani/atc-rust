struct Bit {
    n: usize,
    bit0: Vec<i64>,
    bit1: Vec<i64>,
}
#[allow(dead_code)]
impl Bit {
    fn new(n: usize) -> Self {
        Bit {
            n: n + 1,
            bit0: vec![0; n + 1],
            bit1: vec![0; n + 1],
        }
    }
    fn add_help(&mut self, p: usize, i: usize, x: i64) {
        let mut idx = i as i64;
        while idx < self.n as i64 {
            if p == 0 {
                self.bit0[idx as usize] += x;
            } else {
                self.bit1[idx as usize] += x;
            }
            idx += idx & -idx;
        }
    }
    /// add x to [l, r)
    fn add(&mut self, l: usize, r: usize, x: i64) {
        self.add_help(0, l, -x * (l as i64 - 1));
        self.add_help(1, l, x);
        self.add_help(0, r, x * (r as i64 - 1));
        self.add_help(1, r, -x);
    }
    fn sum_help(&self, p: usize, i: usize) -> i64 {
        let mut idx = i as i64;
        let mut ret = 0;
        while idx > 0 {
            if p == 0 {
                ret += self.bit0[idx as usize];
            } else {
                ret += self.bit1[idx as usize];
            }
            idx -= idx & -idx;
        }
        ret
    }
    // sum of a[1] + a[2] + ... + a[i]
    fn sum(&self, i: usize) -> i64 {
        self.sum_help(0, i) + self.sum_help(1, i) * i as i64
    }
    fn sum_rng(&self, l: usize, r: usize) -> i64 {
        self.sum(r - 1) - self.sum(l - 1)
    }
    fn at(&self, i: usize) -> i64 {
        self.sum_rng(i, i + 1)
    }
}
