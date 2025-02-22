#[allow(dead_code)]
fn format() {
    // format
    println!("{:b}", 1234); // => 10011010010
    println!("{:o}", 1234); // => 2322
    println!("{:x}", 1234); // => 4d2
    println!("{:X}", 1234); // => 4D2
    println!("{:e}", 12.34); // => 1.234e1
    println!("{:E}", 12.34); // => 1.234E1
}
#[allow(dead_code)]
fn max_index(a: &Vec<usize>) -> Option<usize> {
    // maxのindex
    a.iter().enumerate().max_by_key(|x| x.1).map(|x| x.0)
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
        if let Some((hi, wi)) = add_vec(pos, d) {
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
        if let Some((hi, wi)) = add_vec(pos, d) {
            if wi < w && hi < h {
                ret.push((hi, wi))
            }
        }
    }
    ret
}
