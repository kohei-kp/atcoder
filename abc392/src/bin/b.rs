use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; m],
    }

    let mut result = vec![];

    for i in 1..=n {
        let _i = i as i32;
        if !a.contains(&_i) {
            result.push(i);
        }
    }

    println!("{}", result.len());
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
}
