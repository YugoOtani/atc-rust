use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Path {
    d: usize,
    to: usize,
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
fn traveling_salesman(
    st: usize,
    n_nd: usize,
    nei: &Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<Option<(usize, usize)>>> {
    let mut dp: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; 1 << n_nd]; n_nd];
    dp[st][0] = Some((0, 1));
    for hist in 0..1 << n_nd {
        for cur in 0..n_nd {
            if let Some((d_cur, cnt_cur)) = dp[cur][hist] {
                for &(nb, cur_to_nb) in &nei[cur] {
                    if hist & (1 << nb) != 0 {
                        continue;
                    }
                    let hist2 = hist | (1 << nb);
                    match dp[nb][hist2] {
                        Some((d_nb, cnt_nb)) => {
                            if d_nb == d_cur + cur_to_nb {
                                dp[nb][hist2] = Some((d_nb, cnt_nb + cnt_cur))
                            } else if d_nb > d_cur + cur_to_nb {
                                dp[nb][hist2] = Some((d_cur + cur_to_nb, cnt_cur))
                            }
                        }
                        None => dp[nb][hist2] = Some((d_cur + cur_to_nb, cnt_cur)),
                    }
                }
            }
        }
    }
    dp
}
