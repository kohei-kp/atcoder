use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    }

    let mut snakes = vec![];

    for _ in 0..n {
        input! {
            t: i32,
            l: i32,
        }
        snakes.push((t, l));
    }

    for k in 1..=d {
        let mut ans = vec![];
        for (t, l) in snakes.iter() {
            ans.push(t * (l + k as i32));
        }
        println!("{}", ans.iter().max().unwrap());
    }
}
