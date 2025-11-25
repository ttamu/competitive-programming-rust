use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [String; n],
    }

    let grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
    let mut st = BTreeSet::new();
    for i in 0..=n - m {
        for j in 0..=n - m {
            let sub_grid: Vec<Vec<char>> = (i..i + m).map(|x| grid[x][j..j + m].to_vec()).collect();
            st.insert(sub_grid);
        }
    }

    println!("{}", st.len());
}
