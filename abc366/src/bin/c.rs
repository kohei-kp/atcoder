use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut bag = std::collections::HashMap::new();

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    x: usize,
                }
                bag.insert(x, bag.get(&x).unwrap_or(&0) + 1);
            }
            2 => {
                input! {
                    x: usize,
                }
                if let Some(count) = bag.get_mut(&x) {
                    *count -= 1;
                    if *count == 0 {
                        bag.remove(&x);
                    }
                }
            }
            3 => {
                println!("{}", bag.keys().len());
            }
            _ => unreachable!(),
        }
    }
}
