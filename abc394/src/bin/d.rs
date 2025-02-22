use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    println!("{}", if solve(s) { "Yes" } else { "No" });
}

fn solve(s: Vec<char>) -> bool {
    let mut stack = Vec::new();

    for c in s {
        match c {
            ')' => {
                if !pop(&mut stack, '(') {
                    return false;
                }
            }
            ']' => {
                if !pop(&mut stack, '[') {
                    return false;
                }
            }
            '>' => {
                if !pop(&mut stack, '<') {
                    return false;
                }
            }
            _ => {
                stack.push(c);
            }
        };
    }
    stack.is_empty()
}

fn pop(stack: &mut Vec<char>, c: char) -> bool {
    if stack.is_empty() {
        return false;
    }
    if stack.last().unwrap() != &c {
        return false;
    }
    stack.pop();
    true
}
