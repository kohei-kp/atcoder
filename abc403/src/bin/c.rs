use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        q: usize,
    }

    let inf = 1_000_000_000_000_000_000;

    let mut map: HashMap<i64, HashMap<i64, bool>> = HashMap::new();
    for i in 0..n {
        map.entry(i + 1)
            .or_insert_with(HashMap::new)
            .insert(inf, false);
    }

    for _ in 0..q {
        input! {
            q_type: usize,
        }

        match q_type {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let s = map.get_mut(&(x as i64)).unwrap();
                s.entry(y as i64).or_insert(true);
            }
            2 => {
                input! {
                    x: i64,
                }
                let s = map.get_mut(&x).unwrap();
                s.entry(inf).and_modify(|v| *v = true);
            }
            3 => {
                input! {
                    x: i64,
                    y: i64,
                }

                let s = map.get_mut(&(x as i64)).unwrap();
                if s.get(&y) == Some(&true) {
                    println!("Yes");
                } else {
                    if s.get(&inf) == Some(&true) {
                        println!("Yes");
                    } else {
                        println!("No");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
