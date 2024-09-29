struct BinaryTree<T>
where
    T: Ord,
{
    l: TPtr<T>,
    r: TPtr<T>,
    v: T,
    h: usize,
    sz: usize,
}
struct TPtr<T: Ord>(Option<Box<BinaryTree<T>>>);

impl<T: Ord> TPtr<T> {
    #[allow(dead_code)]
    fn is_null(&self) -> bool {
        matches!(&self.0, None)
    }
    #[allow(dead_code)]
    fn empty() -> Self {
        Self(None)
    }
    #[allow(dead_code)]
    fn h(&self) -> usize {
        match &self.0 {
            Some(t) => t.h,
            None => 0,
        }
    }
    #[allow(dead_code)]
    fn sz(&self) -> usize {
        match &self.0 {
            Some(t) => t.sz,
            None => 0,
        }
    }

    #[allow(dead_code)]
    fn h_dfs(&self) -> usize {
        match &self.0 {
            Some(t) => 1 + usize::max(t.r.h_dfs(), t.l.h_dfs()),
            None => 0,
        }
    }
    #[allow(dead_code)]
    fn sz_dfs(&self) -> usize {
        match &self.0 {
            Some(t) => 1 + t.r.sz_dfs() + t.l.sz_dfs(),
            None => 0,
        }
    }
    #[allow(dead_code)]
    fn h_check(&self) -> bool {
        fn helper<T: Ord>(t: &TPtr<T>) -> Option<usize> {
            match &t.0 {
                None => Some(0),
                Some(t) => match (helper(&t.l), helper(&t.r)) {
                    (Some(l), Some(r)) if usize::max(l, r) + 1 == t.h => Some(t.h),
                    _ => None,
                },
            }
        }
        helper(self).is_some()
    }
    #[allow(dead_code)]
    fn sz_check(&self) -> bool {
        fn helper<T: Ord>(t: &TPtr<T>) -> Option<usize> {
            match &t.0 {
                None => Some(0),
                Some(t) => match (helper(&t.l), helper(&t.r)) {
                    (Some(l), Some(r)) if l + r + 1 == t.sz => Some(t.sz),
                    _ => None,
                },
            }
        }
        helper(self).is_some()
    }
    #[allow(dead_code)]
    fn bal_check(&self) -> bool {
        fn helper<T: Ord>(t: &TPtr<T>) -> Option<usize> {
            match &t.0 {
                None => Some(0),
                Some(t) => match (helper(&t.l), helper(&t.r)) {
                    (Some(l), Some(r)) if usize::abs_diff(l, r) <= 1 => Some(usize::max(l, r) + 1),
                    _ => None,
                },
            }
        }
        helper(self).is_some()
    }
    #[allow(dead_code)]
    fn create(v: T, l: Self, r: Self) -> Self {
        let h = usize::max(l.h(), r.h()) + 1;
        let sz = l.sz() + r.sz() + 1;
        let ret = BinaryTree { l, r, v, h, sz };
        Self(Some(Box::new(ret)))
    }
    #[allow(dead_code)]
    fn size(&self) -> usize {
        match &self.0 {
            Some(t) => 1 + t.l.size() + t.r.size(),
            None => 0,
        }
    }
    #[allow(dead_code)]
    fn bal(v: T, l: Self, r: Self) -> Self {
        let (lh, rh) = (l.h(), r.h());
        if usize::abs_diff(lh, rh) <= 1 {
            return Self::create(v, l, r);
        }
        if lh < rh {
            // h(r) - h(l) = 2
            let BinaryTree {
                l: rl,
                r: rr,
                v: rv,
                ..
            } = *r.0.unwrap();
            return if rr.h() >= rl.h() {
                Self::create(rv, Self::create(v, l, rl), rr)
            } else {
                let BinaryTree {
                    l: rll,
                    r: rlr,
                    v: rlv,
                    ..
                } = *rl.0.unwrap();
                Self::create(rlv, Self::create(v, l, rll), Self::create(rv, rlr, rr))
            };
        } else {
            // h(l)-h(r)=2
            let BinaryTree {
                l: ll,
                r: lr,
                v: lv,
                ..
            } = *l.0.unwrap();
            return if ll.h() >= lr.h() {
                Self::create(lv, ll, Self::create(v, lr, r))
            } else {
                let BinaryTree {
                    l: lrl,
                    r: lrr,
                    v: lrv,
                    ..
                } = *lr.0.unwrap();
                Self::create(lrv, Self::create(lv, ll, lrl), Self::create(v, lrr, r))
            };
        }
    }
    #[allow(dead_code)]
    fn contains(&self, t: &T) -> bool {
        match &self.0 {
            None => false,
            Some(tree) => {
                if &tree.v == t {
                    true
                } else if &tree.v < t {
                    tree.r.contains(t)
                } else {
                    tree.l.contains(t)
                }
            }
        }
    }
    #[allow(dead_code)]
    fn insert(self, t: T) -> Self {
        match self.0 {
            None => Self::create(t, Self::empty(), Self::empty()),
            Some(tree) => {
                if &tree.v == &t {
                    return Self(Some(tree));
                }
                let BinaryTree { l, r, v, .. } = *tree;
                if &v < &t {
                    Self::bal(v, l, r.insert(t))
                } else {
                    Self::bal(v, l.insert(t), r)
                }
            }
        }
    }
    #[allow(dead_code)]
    fn insert_mut(&mut self, t: T) {
        let mut tmp = Self::empty();
        std::mem::swap(self, &mut tmp);
        tmp = tmp.insert(t);
        *self = tmp
    }
    #[allow(dead_code)]
    fn min(&self) -> Option<&T> {
        match &self.0 {
            Some(t) => {
                if t.l.is_null() {
                    Some(&t.v)
                } else {
                    t.l.min()
                }
            }
            None => None,
        }
    }
    #[allow(dead_code)]
    fn max(&self) -> Option<&T> {
        match &self.0 {
            Some(t) => {
                if t.r.is_null() {
                    Some(&t.v)
                } else {
                    t.r.max()
                }
            }
            None => None,
        }
    }
    #[allow(dead_code)]
    fn remove_min(self) -> (Self, Option<T>) {
        match self.0 {
            Some(t) => {
                let BinaryTree { l, r, v, .. } = *t;
                if l.is_null() {
                    (r, Some(v))
                } else {
                    let (l, lv) = l.remove_min();
                    let ret = Self::bal(v, l, r);
                    (ret, lv)
                }
            }
            None => (Self::empty(), None),
        }
    }
    #[allow(dead_code)]
    fn merge(self, rhs: Self) -> Self {
        if rhs.is_null() {
            self
        } else {
            let (r, rmin) = rhs.remove_min();
            let rmin = rmin.unwrap();
            Self::bal(rmin, self, r)
        }
    }
    #[allow(dead_code)]
    fn remove(self, t: &T) -> (Self, bool) {
        if self.is_null() {
            (self, false)
        } else {
            let BinaryTree { l, r, v, .. } = *self.0.unwrap();
            if &v == t {
                (Self::merge(l, r), true)
            } else if &v < t {
                let (r, b) = r.remove(t);
                (Self::bal(v, l, r), b)
            } else {
                let (l, b) = l.remove(t);
                (Self::bal(v, l, r), b)
            }
        }
    }
    #[allow(dead_code)]
    fn remove_mut(&mut self, t: &T) -> bool {
        let b;
        let mut tmp = Self::empty();
        std::mem::swap(self, &mut tmp);
        (tmp, b) = tmp.remove(&t);
        *self = tmp;
        b
    }
    #[allow(dead_code)]
    fn traverse(&self, f: &impl Fn(&T) -> ()) {
        match &self.0 {
            None => (),
            Some(t) => {
                t.l.traverse(f);
                f(&t.v);
                t.r.traverse(f);
            }
        }
    }
    #[allow(dead_code)]
    fn lower_bound<'a>(&'a self, lb: &T) -> Option<&'a T> {
        match &self.0 {
            None => None,
            Some(t) => {
                if &t.v == lb {
                    Some(&t.v)
                } else if &t.v < lb {
                    t.r.lower_bound(lb)
                } else {
                    let l = t.l.lower_bound(lb);
                    if l.is_some() {
                        l
                    } else {
                        Some(&t.v)
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    fn upper_bound<'a>(&'a self, ub: &T) -> Option<&'a T> {
        match &self.0 {
            None => None,
            Some(t) => {
                if &t.v == ub {
                    Some(&t.v)
                } else if ub < &t.v {
                    t.l.upper_bound(ub)
                } else {
                    let r = t.r.upper_bound(ub);
                    if r.is_some() {
                        r
                    } else {
                        Some(&t.v)
                    }
                }
            }
        }
    }
}
impl<T: Ord + Debug> Debug for TPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*fn dfs<T: Ord + Debug>(
            t: &TPtr<T>,
            f: &mut std::fmt::Formatter<'_>,
            indent: usize,
        ) -> std::fmt::Result {
            for _ in 0..indent {
                f.write_str("--")?;
            }
            match &t.0 {
                Some(t) => {
                    writeln!(f, "{:?}", &t.v)?;
                    dfs(&t.l, f, indent + 1)?;
                    dfs(&t.r, f, indent + 1)
                }
                None => writeln!(f, "()"),
            }
        }*/
        fn dfs<T: Ord + Debug>(t: &TPtr<T>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &t.0 {
                None => write!(f, "."),
                Some(t) => {
                    write!(f, "(")?;
                    write!(f, "{:?} ", t.v)?;
                    dfs(&t.l, f)?;
                    write!(f, " ")?;
                    dfs(&t.r, f)?;
                    write!(f, ")")
                }
            }
        }
        dfs(self, f)
    }
}
