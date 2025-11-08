use proconio::input;

fn main() {
    input! {
        mut x: i32,
        n: usize,
        w: [i32; n],
        query: usize
    }

    let mut flags = vec![];
    for i in 0..n {
        flags.push(false);
    }

    for i in 0..query {
        input! {
            p: i32
        }

        let num = p - 1;

        if flags[num as usize] {
            x = x - w[num as usize];
            flags[num as usize] = false;
        } else {
            x = x + w[num as usize];
            flags[num as usize] = true;
        }
        println!("{}", x);
    }
}
