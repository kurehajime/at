use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        lines: [Chars; h],
    }

    let map: Vec<char> = lines
        .iter()
        .map(|line| -> String { line.iter().collect::<String>() })
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();
    let start = map.iter().position(|&x| x == 's').unwrap();
    let result = search(map, w, h, start);
    println!("{}", if result { "Yes" } else { "No" });
}
fn search(map: Vec<char>, w: usize, h: usize, start: usize) -> bool {
    let mut stack = Vec::new();
    let mut visited = Vec::new();
    stack.push(start);
    while stack.len() > 0 {
        let i = stack.pop().unwrap();
        if map[i] == 'g' {
            return true;
        }
        let next = dig(&map, w, h, &visited, i);
        visited.extend(next.clone());
        stack.extend(next);
    }
    false
}

fn dig(map: &Vec<char>, w: usize, h: usize, visited: &Vec<usize>, i: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let x = i % w;
    let y = i / w;
    if x > 0 && map[i - 1] != '#' {
        if !visited.iter().any(|&x| x == i - 1) {
            result.push(i - 1);
        }
    }
    if x < w - 1 && map[i + 1] != '#' {
        if !visited.iter().any(|&x| x == i + 1) {
            result.push(i + 1);
        }
    }
    if y > 0 && map[i - w] != '#' {
        if !visited.iter().any(|&x| x == i - w) {
            result.push(i - w);
        }
    }
    if y < h - 1 && map[i + w] != '#' {
        if !visited.iter().any(|&x| x == i + w) {
            result.push(i + w);
        }
    }
    result
}
