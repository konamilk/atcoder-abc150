use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        s: Chars
    };

    let mut ans = 0;

    for i in 0..n-2{
        if s[i] == 'A'
            && s[i+1] == 'B'
            && s[i+2] == 'C'
        {
            ans += 1;
        }
    }

    println!("{}", ans);
}
