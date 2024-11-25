use proconio::input;

fn main() {
    input! {
        max_month: i32,
        max_day: i32,
        y: i32,
        m: i32,
        d: i32,
    }

    // d == max_dayなら1になる
    let _d = if d == max_day { 1 } else { d + 1 };
    // m == max_monthなら1になる,d == max_dayならm+1,d == max_dayでないならmはそのまま
    let _m = if m == max_month && d == max_day {
        1
    } else if d == max_day {
        m + 1
    } else {
        m
    };
    // yearは1/1の時だけ+1する
    let _y = if _d == 1 && _m == 1 { y + 1 } else { y };

    println!("{} {} {}", _y, _m, _d);
}
