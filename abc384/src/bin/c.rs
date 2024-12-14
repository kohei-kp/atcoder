use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }

    let users = vec![
        "ABCDE", "BCDE", "ACDE", "ABDE", "ABCE", "ABCD", "CDE", "BDE", "ADE", "BCE", "ACE", "BCD",
        "ABE", "ACD", "ABD", "ABC", "DE", "CE", "BE", "CD", "AE", "BD", "AD", "BC", "AC", "AB",
        "E", "D", "C", "B", "A",
    ];

    let mut ans = users
        .iter()
        .map(|user| {
            let mut sum = 0;
            user.chars().for_each(|ch| {
                sum += match ch {
                    'A' => a,
                    'B' => b,
                    'C' => c,
                    'D' => d,
                    'E' => e,
                    _ => 0,
                }
            });
            (*user, sum)
        })
        .collect::<Vec<(&str, i32)>>();

    ans.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ans.iter().for_each(|(user, _)| {
        println!("{}", user);
    });
}
