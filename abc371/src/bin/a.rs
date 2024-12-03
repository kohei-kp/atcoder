use proconio::input;

fn main() {
    input! {
        s_ab: String,
        s_ac: String,
        s_bc: String,
    }

    let s = format!("{}{}{}", s_ab, s_ac, s_bc);

    match s.as_str() {
        "<<<" => println!("B"),
        "<<>" => println!("C"),
        "<>>" => println!("A"),
        ">>>" => println!("B"),
        ">><" => println!("C"),
        "><<" => println!("A"),
        _ => unreachable!(),
    }
}
