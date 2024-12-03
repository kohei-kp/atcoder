use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [(i32, String); m],
    }

    let mut taro = HashMap::new();

    for i in 0..m {
        let (x, s) = &a[i];
        if taro.get(x) == None && s == "M" {
            taro.insert(x, s);
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
