/// |ax + by| = 1を満たすx, yを求める.
/// gcd(a,b) = 1が必要
#[allow(dead_code)]
fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    } else {
        let (y, x) = ext_gcd(b, a % b);
        (x, y - a / b * x)
    }
}
#[allow(dead_code)]
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
