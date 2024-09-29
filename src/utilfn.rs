fn util_rs() {
    // maxのindex
    let mx = a.iter().enumerate().max_by_key(|x| x.1);
    println!("{:b}", 1234); // => 10011010010
    println!("{:o}", 1234); // => 2322
    println!("{:x}", 1234); // => 4d2
    println!("{:X}", 1234); // => 4D2
    println!("{:e}", 12.34); // => 1.234e1
    println!("{:E}", 12.34); // => 1.234E1
}
