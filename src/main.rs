#![allow(unused_imports)]
use core::time;
use itertools::*;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::f32::consts::E;
use std::f64::consts::{FRAC_PI_2, PI};
use std::f64::consts::{FRAC_PI_4, LOG2_E};
use std::fmt::{Binary, Debug, Display};
use std::hash::Hash;
use std::io::empty;
use std::io::Write;
use std::ops::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};
#[allow(unused_macros)]
macro_rules! d {
    ( $( $x:expr ),* ) => {
        {
            $(
                print!("{} : {:?} | ", stringify!($x), $x);
            )*
            println!("");
        }
    };
}
#[allow(unused_macros)]
macro_rules! p {
    ( $( $x:expr ),* ) => {
        {
            $(
                print!("{:?} ", $x);
            )*
            println!("");
        }
    };
}
fn main() {}

/// types
#[allow(dead_code)]
type G<T> = Vec<Vec<T>>;
type IntForMint = usize;

/// Utils
struct Util();
struct Dbg();
struct Algo();
#[derive(Debug, Clone)]
struct Path {
    d: usize,
    to: usize,
}

impl Algo {
    #[allow(dead_code)]
    fn prime(n: usize) -> Vec<bool> {
        let mut p = vec![true; n + 1];
        p[0] = false;
        p[1] = false;
        for i in 2..=n {
            if p[i] {
                let mut j = 2 * i;
                while j <= n {
                    p[j] = false;
                    j += i;
                }
            }
        }
        p
    }
    #[allow(dead_code)]
    fn bellman_ford(
        st: usize,
        n_nd: usize,
        edge: &Vec<(usize, usize, i64)>,
    ) -> Option<Vec<Option<i64>>> {
        let mut ret = vec![None; n_nd];
        ret[st] = Some(0);
        for i in 0..n_nd {
            for &(from, to, cost) in edge {
                if ret[from].is_none() {
                    continue;
                }
                let dfrm = ret[from].unwrap();
                if ret[to].is_none() || ret[to].unwrap() > dfrm + cost {
                    ret[to] = Some(dfrm + cost);
                    if i == n_nd - 1 {
                        return None;
                    }
                }
            }
        }
        Some(ret)
    }
    #[allow(dead_code)]
    fn djkstra(st: usize, nb: &Vec<Vec<Path>>) -> Vec<Option<usize>> {
        let n = nb.len();
        let mut q = BinaryHeap::new();
        let mut ret = vec![None; n];
        q.push(Reverse((0, st)));
        ret[st] = Some(0);
        while let Some(Reverse((dcur, cur))) = q.pop() {
            if dcur > ret[cur].unwrap() {
                continue;
            }
            for Path { d, to } in &nb[cur] {
                if ret[*to].is_none() || dcur + d < ret[*to].unwrap() {
                    ret[*to] = Some(dcur + d);
                    q.push(Reverse((dcur + d, *to)));
                }
            }
        }
        ret
    }

    #[allow(dead_code)]
    fn ok_ng<T>(v: &Vec<T>, is_ok: impl Fn(&T) -> bool) -> (usize, usize) {
        if v.len() == 0 {
            return (0, 0);
        }
        if v.len() == 1 {
            if is_ok(&v[0]) {
                return (1, 0);
            } else {
                return (0, 1);
            }
        }
        if !is_ok(&v[0]) {
            return (0, v.len());
        }
        let mut ok = 0;
        let mut ng = v.len();
        while ng - ok > 1 {
            let mid = ok + (ng - ok) / 2;
            if is_ok(&v[mid]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        (ok + 1, v.len() - ng)
    }
    #[allow(dead_code)]
    fn rle<T: Eq>(input: &Vec<T>) -> Vec<(&T, usize)> {
        let mut ret = vec![];
        for x in input {
            match ret.pop() {
                None => ret.push((x, 1)),
                Some((y, c)) => {
                    if x == y {
                        ret.push((x, c + 1))
                    } else {
                        ret.push((y, c));
                        ret.push((x, 1));
                    }
                }
            }
        }
        ret
    }
}
impl Util {
    /// zero^nを求める
    #[allow(dead_code)]
    fn pow<T: Debug>(
        zero: T,
        n: usize,
        pow: impl Fn(&T) -> T,
        fold: (T, impl Fn(T, &T) -> T),
    ) -> T {
        let mut pow2 = vec![zero];
        for i in 0..63 {
            pow2.push(pow(&pow2[i]));
        }

        let mut acc = fold.0;
        let f = &fold.1;
        for i in 0..64 {
            if n & (1 << i) != 0 {
                acc = f(acc, &pow2[i]);
            }
        }
        acc
    }
    #[allow(dead_code)]
    fn to_bit_array(x: usize, max_bit: usize) -> Vec<bool> {
        let mut v = vec![];
        for i in 0..max_bit {
            v.push(x & (1 << i) != 0)
        }
        v
    }
    #[allow(dead_code)]
    fn active_bits(x: usize, max_bit: usize) -> Vec<usize> {
        let mut v = vec![];
        for i in 0..max_bit {
            if x & (1 << i) != 0 {
                v.push(i);
            }
        }
        v
    }
    #[allow(dead_code)]
    fn to_order<T: Ord>(v: &Vec<T>) -> Vec<usize> {
        let n = v.len();
        let mut v = v.iter().enumerate().map(|(i, v)| (i, v)).collect_vec();
        v.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        let mut ret = vec![0; n];
        let mut ord = 0;
        for &(i, _) in &v {
            ret[i] = ord;
            ord += 1;
        }
        ret
    }
    #[allow(dead_code)]
    fn chmax<T: Ord>(t: &mut T, v: T) -> bool {
        if *t < v {
            *t = v;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    fn chmin<T: Ord>(t: &mut T, v: T) -> bool {
        if v < *t {
            *t = v;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    fn chmin_opt<T: Ord>(t: &mut Option<T>, v: Option<T>) -> bool {
        match (t.as_ref(), v) {
            (None, None) | (Some(_), None) => false,
            (None, Some(v)) => {
                *t = Some(v);
                true
            }
            (Some(tt), Some(v)) => {
                if tt > &v {
                    *t = Some(v);
                    true
                } else {
                    false
                }
            }
        }
    }
    #[allow(dead_code)]
    fn chif<T>(t: &mut T, v: T, f: impl Fn(&T, &T) -> bool) -> bool {
        if f(t, &v) {
            *t = v;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    fn dist_pow2(p: (f64, f64), q: (f64, f64)) -> f64 {
        (p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)
    }
    #[allow(dead_code)]
    /// |ax + by| = 1を満たすx, yを求める
    /// gcd(a,b) = 1が必要
    fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            return (1, 0);
        } else {
            let (y, x) = Self::ext_gcd(b, a % b);
            (x, y - a / b * x)
        }
    }
    #[allow(dead_code)]
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    #[allow(dead_code)]
    fn nth_alpha(n: usize) -> char {
        ('a' as u8 + n as u8) as char
    }
    #[allow(dead_code)]
    fn nth_alpha_cap(n: usize) -> char {
        ('A' as u8 + n as u8) as char
    }
    #[allow(dead_code)]
    fn alpha_to_nth(c: char) -> usize {
        (c as u8 - 'a' as u8) as usize
    }
    #[allow(dead_code)]
    fn alpha_to_nth_cap(c: char) -> usize {
        (c as u8 - 'A' as u8) as usize
    }
    #[allow(dead_code)]
    fn some_min<T: Ord>(a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (None, x) | (x, None) => x,
            (Some(a), Some(b)) => Some(if a < b { a } else { b }),
        }
    }
    #[allow(dead_code)]
    fn print_list<T: Display>(a: &[T], sep: &str) {
        let n = a.len();
        for i in 0..n {
            print!("{}", a[i]);
            if i == n - 1 {
                println!("");
            } else {
                print!("{}", sep);
            }
        }
    }

    #[allow(dead_code)]
    fn add_vec(pos: (usize, usize), d: (i32, i32)) -> Option<(usize, usize)> {
        let p0 = pos.0 as i64 + d.0 as i64;
        let p1 = pos.1 as i64 + d.1 as i64;
        if p0 < 0 || p1 < 0 {
            None
        } else {
            Some((p0 as usize, p1 as usize))
        }
    }
    #[allow(dead_code)]
    fn d2(pos: (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(2);
        for d in vec![(0, 1), (1, 0)] {
            if pos.0 + d.0 < h && pos.1 + d.1 < w {
                ret.push((pos.0 + d.0, pos.1 + d.1))
            }
        }
        ret
    }
    #[allow(dead_code)]
    fn d4(pos: (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(4);
        for d in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((hi, wi)) = Self::add_vec(pos, d) {
                if wi < w && hi < h {
                    ret.push((hi, wi))
                }
            }
        }
        ret
    }
    #[allow(dead_code)]
    fn d8(pos: (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(8);
        for d in vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            if let Some((hi, wi)) = Self::add_vec(pos, d) {
                if wi < w && hi < h {
                    ret.push((hi, wi))
                }
            }
        }
        ret
    }
    #[allow(dead_code)]
    fn extract_if<T, F>(v: &mut Vec<T>, mut f: F) -> Vec<T>
    where
        T: Copy,
        F: FnMut(&T) -> bool,
    {
        let mut ind = 0;
        let mut ret = vec![];
        for i in 0..v.len() {
            if f(&v[i]) {
                ret.push(v[i])
            } else {
                v[ind] = v[i];
                ind += 1;
            }
        }
        v.drain(ind..);
        ret
    }

    #[allow(dead_code)]
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
impl Dbg {
    #[allow(dead_code)]
    fn grid_opt<T: Debug>(g: &Vec<Vec<Option<T>>>) {
        println!("----------------------");
        for gi in g {
            for gi in gi {
                match gi {
                    Some(gi) => print!("{:?} ", gi),
                    None => print!("* "),
                }
            }
            println!("")
        }
        println!("----------------------");
    }
    #[allow(dead_code)]
    fn grid<T: Debug>(g: &Vec<Vec<T>>) {
        println!("----------------------");
        for gi in g {
            for gi in gi {
                print!("{:?} ", gi);
            }
            println!("")
        }
        println!("----------------------");
    }
    #[allow(dead_code)]
    fn iter<T: Debug>(ite: impl Iterator<Item = T>) {
        for x in ite {
            print!("{:?} ", x);
        }
        println!("");
    }
}
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
        let iinv = (Util::mod_inv(i as i64, self.md as i64) as IntForMint) % self.md;
        Self {
            i: (self.i * iinv) % self.md,
            md: self.md,
        }
    }
}
struct OSetEntity<T>
where
    T: Ord,
{
    l: OrderedSet<T>,
    r: OrderedSet<T>,
    v: T,
    h: usize,
    sz: usize,
}
struct OrderedSet<T: Ord>(Option<Box<OSetEntity<T>>>);

impl<T: Ord> OrderedSet<T> {
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
        fn helper<T: Ord>(t: &OrderedSet<T>) -> Option<usize> {
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
        fn helper<T: Ord>(t: &OrderedSet<T>) -> Option<usize> {
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
        fn helper<T: Ord>(t: &OrderedSet<T>) -> Option<usize> {
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
        let ret = OSetEntity { l, r, v, h, sz };
        Self(Some(Box::new(ret)))
    }
    #[allow(dead_code)]
    fn bal(v: T, l: Self, r: Self) -> Self {
        let (lh, rh) = (l.h(), r.h());
        if usize::abs_diff(lh, rh) <= 1 {
            return Self::create(v, l, r);
        }
        if lh < rh {
            // h(r) - h(l) = 2
            let OSetEntity {
                l: rl,
                r: rr,
                v: rv,
                ..
            } = *r.0.unwrap();
            return if rr.h() >= rl.h() {
                Self::create(rv, Self::create(v, l, rl), rr)
            } else {
                let OSetEntity {
                    l: rll,
                    r: rlr,
                    v: rlv,
                    ..
                } = *rl.0.unwrap();
                Self::create(rlv, Self::create(v, l, rll), Self::create(rv, rlr, rr))
            };
        } else {
            // h(l)-h(r)=2
            let OSetEntity {
                l: ll,
                r: lr,
                v: lv,
                ..
            } = *l.0.unwrap();
            return if ll.h() >= lr.h() {
                Self::create(lv, ll, Self::create(v, lr, r))
            } else {
                let OSetEntity {
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
                let OSetEntity { l, r, v, .. } = *tree;
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
                let OSetEntity { l, r, v, .. } = *t;
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
            let OSetEntity { l, r, v, .. } = *self.0.unwrap();
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
    fn entry<'a>(&'a mut self, t: &T) -> Option<&'a mut T> {
        match &mut self.0 {
            None => None,
            Some(tree) => {
                if &tree.v == t {
                    Some(&mut tree.v)
                } else if &tree.v < t {
                    tree.r.entry(t)
                } else {
                    tree.l.entry(t)
                }
            }
        }
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
    fn index_of(&self, v: &T) -> Result<usize, usize> {
        match &self.0 {
            None => Err(0),
            Some(t) => {
                if &t.v == v {
                    Ok(t.l.sz())
                } else if v < &t.v {
                    t.l.index_of(v)
                } else {
                    // v > t.v
                    match t.r.index_of(v) {
                        Ok(i) => Ok(t.l.sz() + 1 + i),
                        Err(i) => Err(t.l.sz() + 1 + i),
                    }
                }
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
impl<T: Ord + Debug> Debug for OrderedSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn dfs<T: Ord + Debug>(
            t: &OrderedSet<T>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
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

#[allow(dead_code)]
struct Cmb {
    md: usize,
    fact: Vec<usize>,
}
impl Cmb {
    #[allow(dead_code)]
    fn new(n: usize, md: usize) -> Self {
        let mut v = vec![0; n + 1];
        v[0] = 1;
        for i in 1..=n {
            v[i] = (v[i - 1] * i) % md;
        }
        Cmb { md, fact: v }
    }
    #[allow(dead_code)]
    fn cmb(&self, n: usize, k: usize) -> usize {
        let md = self.md as i64;
        let kinv = Self::inv(self.fact[k] as i64, md);
        let nkinv = Self::inv(self.fact[n - k] as i64, md);
        let n = self.fact[n] as i64;
        ((((n * kinv) % md) * nkinv) % md) as usize
    }
    #[allow(dead_code)]
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

// PASTE OTHER LIBRARY HERE
///////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
