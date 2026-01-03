use proconio::{input, marker::Chars};
fn main() {
    input! {
        mut x: Chars,
    }
    x.sort();

    let mut i = 0;
    while x[0] == '0' {
        x.swap(0, i);
        i += 1;
    }

    println!("{}", x.iter().collect::<String>());
}
