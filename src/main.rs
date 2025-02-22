#![allow(unused_imports)]
use core::time;
use itertools::*;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::fmt::{Binary, Debug, Display};
use std::hash::Hash;
use std::io::Write;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};
use std::{ops::*, vec};
#[allow(unused_macros)]
macro_rules! d { ( $( $x:expr ),* )=> {{$(print!("{} : {:?} | ", stringify!($x), $x);)* println!("");}};}

fn main() {}

#[allow(dead_code)]
fn print_list<T: Display>(a: &[T], sep: &str) {
    let n = a.len();
    for i in 0..n {
        print!("{}", a[i]);
        if i == n - 1 {
            println!("");
        } else {
            print!("{}", sep);
        }
    }
}

#[allow(dead_code)]
fn print_grid<T: Debug>(g: &Vec<Vec<T>>) {
    for gi in g {
        for gi in gi {
            print!("{:?} ", gi);
        }
        println!("")
    }
}
