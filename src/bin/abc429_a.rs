use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    for i in 1..=n {
        println!("{}", if i <= m { "OK" } else { "Too Many Requests" });
    }
}
