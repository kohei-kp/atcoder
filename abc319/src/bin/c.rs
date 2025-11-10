use itertools::Itertools;
use proconio::input;

fn main() {
    let mut nums = vec![];

    for i in 0..3 {
        input! {
            row: [usize; 3]
        }
        nums.extend(row);
    }

    let perms = (0usize..9).permutations(9);

    let mut cnt = 0;
    let mut tot = 0;
    for comb in perms {
        let mut ok = true;

        let f =
            |i: usize, j: usize, k: usize, comb: &Vec<usize>, nums: &Vec<usize>, ok: &mut bool| {
                let mut d = vec![(comb[i], nums[i]), (comb[j], nums[j]), (comb[k], nums[k])];
                d.sort_by_key(|x| x.0);
                if d[0].1 == d[1].1 {
                    *ok = false;
                }
            };

        // チェック
        f(0, 1, 2, &comb, &nums, &mut ok);
        f(3, 4, 5, &comb, &nums, &mut ok);
        f(6, 7, 8, &comb, &nums, &mut ok);
        f(0, 3, 6, &comb, &nums, &mut ok);
        f(1, 4, 7, &comb, &nums, &mut ok);
        f(2, 5, 8, &comb, &nums, &mut ok);
        f(0, 4, 8, &comb, &nums, &mut ok);
        f(2, 4, 6, &comb, &nums, &mut ok);

        if ok {
            cnt += 1
        }
        tot += 1;
    }

    let ans = cnt as f64 / tot as f64;
    println!("{:.10}", ans);
}
