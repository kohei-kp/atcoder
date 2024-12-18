use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut map = HashMap::new();

    let mut ans = vec![];

    for i in 0..n {
        let b = a[i];

        if map.contains_key(&b) {
            ans.push(map.get(&b).unwrap() + 1);
        } else {
            ans.push(-1);
        }
        map.insert(b, i as i32);
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
