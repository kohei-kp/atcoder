use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut heads: [i64; n],
        mut bodies: [i64; m],
    }

    heads.sort_by(|a, b| a.cmp(b));
    bodies.sort_by(|a, b| a.cmp(b));

    let mut head_index = 0;
    let mut body_index = 0;
    let mut ans = 0;
    while head_index < n && body_index < m {
        if heads[head_index] <= bodies[body_index] {
            ans += 1;
            head_index += 1;
            body_index += 1;
        } else {
            body_index += 1;
        }
        if ans >= k {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
