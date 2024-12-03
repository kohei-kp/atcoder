use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut data = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..=i {
            input! {
                num: usize
            }
            data[i][j] = num;
            data[i][j] -= 1;
            data[j][i] = data[i][j];
        }
    }
    let mut x = 0;

    for i in 0..n {
        x = data[x][i];
    }
    println!("{}", x + 1);
}
