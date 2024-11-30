use proconio::input;

fn main() {
    input! {
        n: i32,
        q: usize,
        h: [(String, i32); q],
    }

    let mut left = 1;
    let mut right = 2;

    let mut move_count = 0;
    for i in 0..q {
        if h[i].0 == "L" {
            move_count += num_move(n, left, h[i].1, right);
            left = h[i].1;
        } else {
            move_count += num_move(n, right, h[i].1, left);
            right = h[i].1;
        }
    }

    println!("{}", move_count);
}

fn num_move(n: i32, from: i32, to: i32, ng: i32) -> i32 {
    let mut _from = from;
    let mut _to = to;
    if from > to {
        std::mem::swap(&mut _from, &mut _to);
    }

    if _from < ng && ng < _to {
        return n + _from - _to;
    } else {
        return _to - _from;
    }
}
