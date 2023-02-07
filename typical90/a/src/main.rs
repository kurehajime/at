use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }
    let mut kireme: Vec<usize> = vec![0; n - 1];
    for i in 1..n {
        kireme[i - 1] = a[i] - a[i - 1];
    }

    println!("{:?}", kireme);
}
