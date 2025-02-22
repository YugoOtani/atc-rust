/// zero^nを求める
#[allow(dead_code)]
fn pow<T>(zero: T, n: usize, pow: impl Fn(&T) -> T, fold: (T, impl Fn(T, &T) -> T)) -> T {
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
