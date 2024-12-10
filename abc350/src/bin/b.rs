use proconio::input;

fn main() {
    input! {
        mut n: i32,
        q: usize,
        t: [i32; q],
    }

    let mut checked = std::collections::HashSet::new();

    for i in 0..q {
        if checked.contains(&t[i]) {
            n += 1;
            checked.remove(&t[i]);
        } else {
            n -= 1;
            checked.insert(t[i]);
        }
    }

    println!("{}", n);
}
