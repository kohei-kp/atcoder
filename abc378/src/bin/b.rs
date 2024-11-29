use proconio::input;

fn main() {
    input! {
        n: usize,
        q_arr: [(i32, i32); n],
        q: usize,
        t_arr: [(usize ,i32); q],
    }

    for (kinds, day) in t_arr {
        let mut _day = day;

        loop {
            let r = _day % q_arr[kinds - 1].0;
            if r == q_arr[kinds - 1].1 {
                break;
            }

            let diff = q_arr[kinds - 1].1 - r;
            if diff < 0 {
                _day += q_arr[kinds - 1].0 - r;
            } else {
                _day += diff;
            }
        }
        println!("{}", _day);
    }
}
