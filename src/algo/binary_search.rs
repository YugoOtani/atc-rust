#[allow(dead_code)]
fn ok_ng<T>(v: &[T], is_ok: impl Fn(&T) -> bool) -> (usize, usize) {
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
