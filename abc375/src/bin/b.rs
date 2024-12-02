use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pos: [(f64, f64); n],
    }

    let base_pos = (0.0, 0.0);
    pos.insert(0, base_pos);
    pos.push(base_pos);

    let mut prev = pos[0];
    let mut ans = 0.0;
    for i in 1..pos.len() {
        let current = pos[i];
        let cost = ((prev.0 - current.0).powi(2) + (prev.1 - current.1).powi(2)).sqrt();
        prev = current;
        ans += cost;
    }

    println!("{}", ans);
}
