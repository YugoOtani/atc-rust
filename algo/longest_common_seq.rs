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
