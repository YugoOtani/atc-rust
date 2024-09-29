fn traveling_salesman(
    st: usize,
    n_nd: usize,
    nei: &Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<Option<(usize, usize)>>> {
    let mut dp: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; 1 << n_nd]; n_nd];
    dp[st][0] = Some((0, 1));
    for vs in 0..1 << n_nd {
        for cur in 0..n_nd {
            if let Some((d_cur, cnt_cur)) = dp[cur][vs] {
                for &(nb, d) in &nei[cur] {
                    if vs & (1 << nb) != 0 {
                        continue;
                    }
                    let vs2 = vs | (1 << nb);
                    match dp[nb][vs2] {
                        Some((dnb, cnt_nb)) => {
                            if dnb == d_cur + d {
                                dp[nb][vs2] = Some((dnb, cnt_nb + cnt_cur))
                            } else if dnb > d_cur + d_nb {
                                dp[nb][vs2] = Some((d_cur + d, cnt_cur))
                            }
                        }
                        None => dp[nb][vs2] = Some((dcur + cur_nb, cnt_cur)),
                    }
                }
            }
        }
    }
    dp
}
