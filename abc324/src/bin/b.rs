use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }

    if n == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

// Powでやるなら
// fn main() {
//     input! {
//         n: i64,
//     }
//
//     for x in 0..60 {
//         let p2 = 2i64.pow(x);
//         if p2 > n {
//             break;
//         }
//
//         let mut p = p2;
//         for _y in 0..60 {
//             if p == n {
//                 println!("Yes");
//                 return;
//             }
//             if p > n / 3 {
//                 break;
//             }
//             p *= 3;
//         }
//     }
//
//     println!("No");
// }
