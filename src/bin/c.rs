use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use itertools::Itertools;

fn main() {
    // let source = AutoSource::from("8
    // 7 3 5 4 2 1 6 8
    // 3 8 2 5 4 6 7 1");
    input!{
        // from source,
        n: usize,
        p: [usize; n],
        q: [usize; n]
    };

    // for x in (0..n).permutations(3){
    //     println!("{:?}", x);
    // }

    let mut pers = vec![];
    let mut buf = vec![0usize;n];
    dfs(&mut pers, &mut buf, 0, (1..=n).into_iter().collect::<Vec<usize>>());

    let mut a= 0;
    let mut b= 0;

    for (i, v) in pers.iter().enumerate(){
        if *v == p{
            a = i as i32
        }
        if *v == q {
            b = i as i32
        }
    }

    println!("{}", (a-b).abs())

}

// results: 完成した順列
// buf: (その時点での）順列
// d: (その時点での)深さ
// remain: 使用していない数値
fn dfs(results: &mut Vec<Vec<usize>>, buf:&mut Vec<usize>, d: usize, remain: Vec<usize>){
    if d == buf.len(){
        // print!("{:?}", buf);
        results.push(buf.clone());
    }
    else {
        for i in 0..remain.len(){
            buf[d] = remain[i];
            let mut next = vec![];
            for (j, &x) in remain.iter().enumerate() {
                if i == j{
                    continue
                }
                next.push(x);
            }
            dfs(results, buf, d+1, next)
        }
    }
}

#[test]
fn test_permutations(){
    let n= 3;
    let mut results =vec![];
    let mut buf = vec![0; n];
    dfs(&mut results, &mut buf, 0, (0..n).into_iter().collect::<Vec<usize>>());
    assert_eq!(results, [[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]]);

    let n= 4;
    let k= 2;
    let mut results =vec![];
    let mut buf = vec![0; k];
    dfs(&mut results, &mut buf, 0, (0..n).into_iter().collect::<Vec<usize>>());
    assert_eq!(results, [[0,1],[0,2],[0,3],[1,0],[1,2],[1,3],[2,0],[2,1],[2,3],[3,0],[3,1],[3,2]])
}
