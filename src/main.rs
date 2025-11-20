use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [usize; n],
        mut b: [usize; m],
    }
    h.sort();
    b.sort();

    let mut cnt = 0;
    let mut idx = 0;
    for &hi in &h {
        while idx < m && b[idx] < hi {
            idx += 1;
        }
        if idx < m {
            cnt += 1;
            idx += 1;
        }
    }

    println!("{}", if cnt >= k { "Yes" } else { "No" });
}
