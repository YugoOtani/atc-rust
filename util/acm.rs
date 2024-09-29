struct Acm2<T: Add + Sub + Default> {
    acm: Vec<Vec<T>>,
}
impl<T: Add + Sub + Default> Acm2<T> {
    fn new(h: usize, w: usize, v: &Vec<Vec<T>>) -> Self {
        let mut acm = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                acm[i + 1][j + 1] = acm[i + 1][j] + v[i][j];
            }
        }
        for j in 1..=w {
            for i in 0..h {
                acm[i + 1][j] = acm[i + 1][j] + acm[i][j];
            }
        }
        Self { acm }
    }
    /// x,y両方含む
    fn sum(&self, x: (usize, usize), y: (usize, usize)) -> Int {
        // x.0, x.1 ... x.0, y.1
        // ..
        // y.0, x.1 ..  y.0, y.1
        let v = &self.acm;
        (v[y.0 + 1][y.1 + 1] - v[x.0][y.1 + 1]) - (v[y.0 + 1][x.1] - v[x.0][x.1])
    }
}
