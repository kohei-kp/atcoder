use proconio::input;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    input! {
        n: usize,
        p: [(i32, i32); n],
    }

    for i in 0..n {
        let mut ans: Vec<i32> = vec![];
        for j in 0..n {
            let dist = calc_euclid(
                Point {
                    x: p[i].0,
                    y: p[i].1,
                },
                Point {
                    x: p[j].0,
                    y: p[j].1,
                },
            );
            ans.push(dist);
        }

        let max_index = ans
            .iter()
            .position(|&x| x == *ans.iter().max().unwrap())
            .unwrap();
        println!("{}", max_index + 1);
    }
}

fn calc_euclid(p1: Point, p2: Point) -> i32 {
    let x = p1.x - p2.x;
    let y = p1.y - p2.y;

    x * x + y * y
}
