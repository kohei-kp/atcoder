use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut front = 0;
    let mut back = 0;
    let mut tx: i64 = 0;
    let mut x = vec![0; q + 5];
    let mut l = vec![0; q + 5];

    for _ in 0..q {
        input! {
            query_type: i32,
        }

        match query_type {
            1 => {
                input! {
                    query: i64
                }
                x[back] = tx;
                l[back] = query;
                back += 1;
                tx += query;
            }
            2 => {
                front += 1;
            }
            3 => {
                input! {
                    mut query: usize
                }
                query -= 1;
                let ans = x[front + query] - x[front];
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
