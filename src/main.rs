use datastructures::*;
enum token<'a> {
    OPERATOR(&'a str),
    OPERAND(i32),
}

fn main() {
    let mut stack = datastructures::Stack::new(None);
    let expression = "4 55 + 62 23 - * ";
    let mut converted = expression
        .split_whitespace()
        .map(|c| match c.parse::<i32>() {
            Ok(v) => token::OPERAND(v),
            Err(_) => token::OPERATOR(c),
        })
        .rev()
        .collect::<Vec<token>>();

    while !converted.is_empty() {
        match converted.pop() {
            Some(token::OPERATOR(val2)) => {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                match val2 {
                    "+" => {
                        stack.push(op1 + op2);
                    }
                    "-" => {
                        stack.push(op1 - op2);
                    }
                    "*" => {
                        stack.push(op1 * op2);
                    }
                    _ => {}
                }
            }
            Some(token::OPERAND(val)) => {
                stack.push(val);
            }
            _ => {}
        }
    }
    println!("end result: {:?}", stack);
}
