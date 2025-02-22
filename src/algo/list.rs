use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;

#[allow(dead_code)]
fn longest_common_seq<T: Eq>(s: &Vec<T>, t: &Vec<T>) -> usize {
    let (sn, tn) = (s.len(), t.len());
    let mut dp = vec![vec![0; tn + 1]; sn + 1];
    for si in 0..sn {
        for ti in 0..tn {
            if s[si] == t[ti] {
                dp[si + 1][ti + 1] = dp[si][ti] + 1;
            }
            dp[si + 1][ti + 1] = usize::max(dp[si + 1][ti + 1], dp[si][ti + 1]);
            dp[si + 1][ti + 1] = usize::max(dp[si + 1][ti + 1], dp[si + 1][ti]);
        }
    }
    dp[sn][tn]
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
fn compress_array(v: &Vec<usize>) -> HashMap<usize, usize> {
    let mut st = BTreeSet::new();
    for &vi in v {
        st.insert(vi);
    }
    let mut mp = HashMap::new();
    let mut cnt = 0;
    for v in st.iter() {
        if !mp.contains_key(v) {
            mp.insert(*v, cnt);
            cnt += 1;
        }
    }
    mp
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
#[allow(dead_code)]
fn topological_sort<T: Eq + Hash>(ord: &Vec<(T, T)>) -> Vec<&T> {
    let mut cnt = HashMap::new();
    let mut to = HashMap::new();
    for (v1, v2) in ord {
        cnt.insert(v1, 0);
        cnt.insert(v2, 0);
        to.insert(v1, vec![]);
        to.insert(v2, vec![]);
    }
    for (v1, v2) in ord {
        *cnt.get_mut(v2).unwrap() += 1;
        to.get_mut(v1).unwrap().push(v2);
    }
    let mut ret = vec![];
    let mut q = VecDeque::new();
    for (k, v) in &cnt {
        if *v == 0 {
            q.push_back(*k);
        }
    }
    while let Some(v) = q.pop_front() {
        ret.push(v);
        for &vto in to.get(v).unwrap() {
            *cnt.get_mut(vto).unwrap() -= 1;
            if cnt.get(vto).unwrap() == &0 {
                q.push_back(vto);
            }
        }
    }
    ret
}
