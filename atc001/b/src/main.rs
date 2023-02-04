use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        querys: [[usize;3];q],
    }

    let mut node = vec![0; n];
    let mut rank: Vec<usize> = vec![0; n];
    init(&mut node);

    for i in 0..q {
        if querys[i][0] == 0 {
            unite(&mut node, &mut rank, querys[i][1], querys[i][2]);
        }
        if querys[i][0] == 1 {
            if some(&mut node, querys[i][1], querys[i][2]) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
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
fn some(node: &mut Vec<usize>, x: usize, y: usize) -> bool {
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
