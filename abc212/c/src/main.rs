use proconio::input;
use proconio::marker::Chars;
use std::time::Instant;
fn main() {
    input! {
        n: usize,
        m: usize,
        narr: [usize; n],
        marr: [usize; m],
    }
    let s = Instant::now();

    println!("{:?}", narr);
    println!("{:?}", marr);

    let e = s.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        e.as_secs(),
        e.subsec_nanos() / 1_000_000
    );
}
