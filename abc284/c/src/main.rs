use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        querys: [[usize;2];m],
    }

    let mut node = vec![0; n + 1];
    let mut rank: Vec<usize> = vec![0; n + 1];
    init(&mut node);

    for i in 0..m {
        unite(&mut node, &mut rank, querys[i][0], querys[i][1]);
    }
    node = node[1..].to_vec();
    node.sort();
    node.dedup();

    println!("{:?}", node);
}

fn init(node: &mut Vec<usize>) {
    for i in 0..node.len() {
        node[i] = i;
    }
}
fn root(node: &mut Vec<usize>, x: usize) -> usize {
    if node[x] == x {
        x
    } else {
        node[x] = root(node, node[x]);
        node[x]
    }
}
fn _some(node: &mut Vec<usize>, x: usize, y: usize) -> bool {
    let rx = root(node, x);
    let ry = root(node, y);
    rx == ry
}
fn unite(node: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = root(node, x);
    let y = root(node, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        node[x] = y;
    } else {
        node[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}
