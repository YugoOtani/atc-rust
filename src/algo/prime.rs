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
