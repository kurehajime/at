use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        narr: [isize; n],
        marr: [isize; m],
    }
    let marr = marr.iter().map(|x| -1 * x).collect();
    let mut all = [narr, marr].concat();
    all.sort_by(|a, b| a.abs().cmp(&b.abs()));

    println!("{:?}", min_diff(all));
}
fn min_diff(all: Vec<isize>) -> isize {
    let mut min = isize::MAX;
    let first = all[0];
    let mut pre_turn = first > 0;
    let mut pre_num = first;
    for i in 1..all.len() {
        let turn = all[i] > 0;
        let num = all[i];
        if pre_turn != turn {
            let diff = (pre_num.abs() - num.abs()).abs();
            if diff < min {
                min = diff;
            }
        }
        pre_turn = turn;
        pre_num = num;
    }

    min
}
