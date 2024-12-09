use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; m],
    }

    let mut x = vec![vec![0; m]; n];
    for i in 0..n {
        input! {
            row: [i32; m],
        }
        x[i][..m].copy_from_slice(&row);
    }

    let mut ans: Vec<i32> = vec![0; m];
    for i in 0..n {
        let row = &x[i];
        for j in 0..m {
            ans[j] += row[j];
        }
    }

    let mut all_satisfy = true;
    for i in 0..m {
        if ans[i] < a[i] {
            all_satisfy = false;
            break;
        }
    }

    println!("{}", if all_satisfy { "Yes" } else { "No" });
}
