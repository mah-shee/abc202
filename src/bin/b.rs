#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut s: Chars
    }
    let ans: String = s
        .iter()
        .map(|&a| match a {
            '6' => '9',
            '9' => '6',
            _ => a,
        })
        .rev()
        .collect();
    println!("{}", ans);
}
