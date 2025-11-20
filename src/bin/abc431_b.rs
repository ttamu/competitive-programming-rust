use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
       x: usize,
       n: usize,
       w: [usize; n],
       q: usize,
       p: [Usize1; q],
    }

    let mut attached = vec![false; n];
    let mut weight = x;

    for &idx in &p {
        if attached[idx] {
            weight -= w[idx];
        } else {
            weight += w[idx];
        }
        attached[idx] = !attached[idx];
        println!("{}", weight);
    }
}
