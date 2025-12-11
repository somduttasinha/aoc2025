use std::panic;

use crate::common::{self, Part};

#[derive(Debug, Copy, Clone)]
enum Token {
    Number(usize),
    Operation(Operator),
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Multiplication = 0,
    Addition = 1,
}

pub fn day6(part: Part) {
    let path = "src/resources/day6.txt";
    let raw = common::read(path);
    let res = run(raw, part);
    println!("res: {}", res);
}

fn run(raw: String, part: Part) -> usize {
    // posiiton of the digit is important
    let mut lines = Vec::<Vec<Token>>::new();
    for line in raw.lines() {
        let mut token_line = Vec::<Token>::new();
        for raw_token in line.split_whitespace() {
            let token = match raw_token.parse::<usize>() {
                Ok(num) => Token::Number(num),
                Err(_) => match raw_token {
                    "+" => Token::Operation(Operator::Addition),
                    "*" => Token::Operation(Operator::Multiplication),
                    _ => panic!("Unexpected token: {}", raw_token),
                },
            };
            token_line.push(token);
        }
        lines.push(token_line);
    }

    let line_length = lines[0].len();

    let res = (0..line_length)
        .map(|i| lines.iter().map(move |l| l[i]).collect::<Vec<Token>>())
        .map(|l| operate(&l))
        .sum::<usize>();

    res
}

fn operate(vertical_line: &Vec<Token>) -> usize {
    let numbers = vertical_line[..vertical_line.len() - 1]
        .iter()
        .map(|t| match t {
            Token::Number(x) => *x,
            _ => panic!("Expected number everywhere"),
        })
        .collect::<Vec<usize>>();

    let operator = match vertical_line.last().expect("Expected operator") {
        Token::Operation(o) => o,
        Token::Number(_) => panic!("Did not expect a number at the end of the line"),
    };

    let result = match operator {
        Operator::Multiplication => numbers.iter().fold(1, |x, y| x * y),
        Operator::Addition => numbers.iter().fold(0, |x, y| x + y),
    };

    result
}

#[test]
fn test_example_part_1() {
    let raw = "123 328  51 64 \n45 64  387 23 \n6 98  215 314\n*   +   *   +  ";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 4277556)
}

#[test]
fn test_example_part_2() {
    let raw = "123 328  51 64 \n45 64  387 23 \n6 98  215 314\n*   +   *   +  ";
    let res = run(raw.to_string(), Part::Part2);
    assert_eq!(res, 3263827)
}

#[test]
fn test_operate() {
    let line = vec![
        Token::Number(123),
        Token::Number(45),
        Token::Number(6),
        Token::Operation(Operator::Multiplication),
    ];
    let res = operate(&line);

    assert_eq!(res, 33210)
}
