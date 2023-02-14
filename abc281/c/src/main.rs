use proconio::input;
fn main() {
    input! {
        n: usize,
        t: u64,
        songs: [u64;n],
    }
    let sum = songs.iter().fold(0, |sum, i| sum + i);
    let end_time = t % sum;
    let mut time: u64 = 0;
    let mut prev_time: u64 = 0;
    let mut index = 0;
    let mut prev_index = 0;

    while time < end_time {
        prev_time = time;
        time += songs[index];
        prev_index = index;
        index = (index + 1) % n;
    }
    println!("{} {}", prev_index + 1, end_time - prev_time);
}
