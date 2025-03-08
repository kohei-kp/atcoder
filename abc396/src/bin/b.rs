use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut cards = [0 as i64; 100].to_vec();

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: i64,
            }
            cards.push(x);
        } else {
            let last = cards.pop();
            println!("{}", last.unwrap());
        }
    }
}
