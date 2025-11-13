use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; k],
        l: [usize; q]
    }

    let mut cell = vec![""; n];
    for i in 0..k {
        cell[a[i] - 1] = "#"
    }

    for i in 0..q {
        let num = l[i];
        let mut cnt = 0;
        for j in 0..cell.len() {
            if cell[j] == "#" {
                cnt += 1;
            }
            if cnt == num {
                if j + 1 < n {
                    if cell[j + 1] == "" {
                        cell[j + 1] = "#";
                        cell[j] = "";
                    }
                }
            }
        }
    }

    let mut cnt = 1;
    for i in 0..cell.len() {
        if cell[i] == "#" {
            print!("{} ", cnt);
        }
        cnt += 1;
    }
    println!("");
}
