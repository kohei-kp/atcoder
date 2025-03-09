use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

    for _ in 0..m {
        input! {
            mut a: usize,
            mut b: usize,
            w: i64,
        }
        a -= 1;
        b -= 1;
        graph[a].push((b, w));
        graph[b].push((a, w));
    }

    let mut ans = i64::MAX;
    let mut stack = vec![(0, vec![false; n], 0)];

    while let Some((node, mut visited, x)) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;

        if node == n - 1 {
            ans = std::cmp::min(ans, x);
            continue;
        }

        for &(next, w) in &graph[node] {
            if !visited[next] {
                stack.push((next, visited.clone(), x ^ w));
            }
        }
    }

    println!("{}", ans);
}
