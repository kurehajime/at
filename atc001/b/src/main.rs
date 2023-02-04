use proconio::input;
use std::time::Instant;
fn main() {
    input! {
        n: i32,
        q: i32,
        querys: [[i32;3];q],
    }
    let s = Instant::now();

    println!("{}", n);
    println!("{}", q);
    println!("{}", querys[8][2]);
    let e = s.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        e.as_secs(),
        e.subsec_nanos() / 1_000_000
    );
}
