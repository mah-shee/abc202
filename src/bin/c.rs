#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize; n],
        b:[usize;n],
        c:[usize;n],
    }
    let mut count = vec![0; n];
    for i in 0..n {
        count[b[c[i] - 1] - 1] += 1;
    }
    let mut ans: u128 = 0;
    for i in 0..n {
        ans += count[a[i] - 1]
    }
    println!("{}", ans);
}
