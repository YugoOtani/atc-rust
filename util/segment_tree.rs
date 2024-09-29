#[allow(dead_code)]
type F<T> = Box<dyn Fn(T, T) -> T>;
#[allow(dead_code)]
type P<T> = Box<dyn Fn(T) -> bool>;
#[allow(dead_code)]
struct SegmentTree<T: Copy + PartialEq + Eq> {
    tree: Vec<T>,
    f: F<T>,
    x: usize,
    default: T,
}
impl<T: Copy + PartialEq + Eq> SegmentTree<T> {
    #[allow(dead_code)]
    fn new(size: usize, default: T, f: F<T>) -> Self {
        let mut i = 1;
        while i < size {
            i *= 2;
        }
        let tree = vec![default; 2 * i - 1];
        Self {
            f,
            x: i,
            tree,
            default,
        }
    }
    #[allow(dead_code)]
    fn init_with(&mut self, v: &Vec<T>, blank: T) {
        let n = v.len();
        for i in 0..n {
            self.set_i_no_build(i, v[i]);
        }
        for i in n..self.x {
            self.set_i_no_build(i, blank);
        }
        self.build();
    }
    #[allow(dead_code)]
    fn build(&mut self) {
        let x = self.x;
        for i in (0..=x - 2).rev() {
            self.tree[i] = (&self.f)(self.tree[2 * i + 1], self.tree[2 * i + 2]);
        }
    }
    #[allow(dead_code)]
    fn get_i(&self, i: usize) -> T {
        self.tree[i + self.x - 1]
    }
    #[allow(dead_code)]
    fn set_i(&mut self, i: usize, v: T) {
        let mut i = i + self.x - 1;
        self.tree[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.tree[i] = (&self.f)(self.tree[2 * i + 1], self.tree[2 * i + 2]);
        }
    }
    #[allow(dead_code)]
    fn set_i_no_build(&mut self, i: usize, v: T) {
        self.tree[i + self.x - 1] = v;
    }
    #[allow(dead_code)]
    fn get_rng(&self, l: usize, r: usize) -> T {
        self._get_rng_inner(l, r, 0, 0, self.x)
    }
    #[allow(dead_code)]
    fn _get_rng_inner(&self, l: usize, r: usize, cur: usize, cur_l: usize, cur_r: usize) -> T {
        if cur_r <= l || r <= cur_l {
            self.default
        } else if l <= cur_l && cur_r <= r {
            self.tree[cur]
        } else {
            let vl = self._get_rng_inner(l, r, cur * 2 + 1, cur_l, (cur_r + cur_l) / 2);
            let vr = self._get_rng_inner(l, r, cur * 2 + 2, (cur_l + cur_r) / 2, cur_r);
            (&self.f)(vl, vr)
        }
    }
    #[allow(dead_code)]
    fn search_left(&self, l: usize, r: usize, pred: &impl Fn(T) -> bool) -> Option<usize> {
        self._search_left_inner(l, r, 0, 0, self.x, pred)
    }
    #[allow(dead_code)]
    fn _search_left_inner(
        &self,
        l: usize,
        r: usize,
        cur: usize,
        cur_l: usize,
        cur_r: usize,
        pred: &impl Fn(T) -> bool,
    ) -> Option<usize> {
        if !pred(self.tree[cur]) || cur_r <= l || r <= cur_l {
            None
        } else if cur >= self.x - 1 {
            Some(cur - self.x + 1)
        } else {
            let tmp = self._search_left_inner(l, r, cur * 2 + 1, cur_l, (cur_l + cur_r) / 2, pred);
            if tmp != None {
                tmp
            } else {
                self._search_left_inner(l, r, cur * 2 + 2, (cur_l + cur_r) / 2, cur_r, pred)
            }
        }
    }
    #[allow(dead_code)]
    fn search_right(&self, l: usize, r: usize, pred: &impl Fn(T) -> bool) -> Option<usize> {
        self._search_right_inner(l, r, 0, 0, self.x, pred)
    }
    #[allow(dead_code)]
    fn _search_right_inner(
        &self,
        l: usize,
        r: usize,
        cur: usize,
        cur_l: usize,
        cur_r: usize,
        pred: &impl Fn(T) -> bool,
    ) -> Option<usize> {
        if !pred(self.tree[cur]) || cur_r <= l || r <= cur_l {
            None
        } else if cur >= self.x - 1 {
            Some(cur - self.x + 1)
        } else {
            let tmp = self._search_right_inner(l, r, cur * 2 + 2, (cur_l + cur_r) / 2, cur_r, pred);
            if tmp != None {
                tmp
            } else {
                self._search_right_inner(l, r, cur * 2 + 1, cur_l, (cur_l + cur_r) / 2, pred)
            }
        }
    }
}
impl<T: Copy + PartialEq + Eq + Display> Display for SegmentTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.x {
            write!(f, "{} ", self.tree[i + self.x - 1])?;
        }
        write!(f, "]")
    }
}
impl<T: Copy + PartialEq + Eq + Debug> Debug for SegmentTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut p = 1;
        let mut i = 0;
        while i < 2 * self.x - 1 {
            write!(f, "--[")?;
            for j in 0..p {
                if j == p - 1 {
                    write!(f, "{:?}]", self.tree[i])?;
                } else {
                    write!(f, "{:?},", self.tree[i])?;
                }
                i += 1;
            }
            p *= 2;
        }
        Ok(())
    }
}
