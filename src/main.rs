use std::collections::HashMap;
use std::io::Write;
use std::num::Wrapping;
use std::time::SystemTime;

const MAX_ITER: u32 = 1000000;

#[derive(Debug, Eq, PartialEq)]
enum BfToken {
    INC, DEC, LEF, RIG, JUM, BAC, ACC, OUT, NAN
}

impl From<char> for BfToken {
    fn from(value: char) -> Self {
        match value {
            '>'=>Self::RIG,
            '<'=>Self::LEF,
            '+'=>Self::INC,
            '-'=>Self::DEC,
            '.'=>Self::OUT,
            ','=>Self::ACC,
            '['=>Self::JUM,
            ']'=>Self::BAC,
            _ => Self::NAN
        }
    }
}

impl BfToken {
    fn from_source(code: &str) -> (Vec<Self>, HashMap<usize, usize>) {
        let start = SystemTime::now();
        let tokens: Vec<BfToken> = code.chars().filter_map(|c| match BfToken::from(c) {BfToken::NAN=>None, T=>Some(T)}).collect();
        let mut jumps = HashMap::<usize, usize>::new();
        let mut queue = vec![];
        for (idx, token) in tokens.iter().enumerate() {
            match token {
                BfToken::JUM => queue.push(idx),
                BfToken::BAC => {
                    let temp = queue.pop().expect(&format!("Unexpected character at {idx}"));
                    jumps.insert(temp, idx);
                    jumps.insert(idx, temp);
                }
                _ => ()
            }
        }
        if !queue.is_empty() {panic!("Unclosed brackets at {queue:?}")}
        println!("Compilation time: {:?}", SystemTime::now().duration_since(start).unwrap());
        (tokens, jumps)
    }
}

fn parse(code: &str) {
    let mut stack = vec![0];
    let (tokens, jumps) = BfToken::from_source(code);
    let mut pointer = 0;
    let mut idx = 0;

    let mut input: Vec<u8> = vec![];
    if tokens.contains(&BfToken::ACC) {
        println!("Enter the input string for the code: ");
        let mut string = "".to_string();
        std::io::stdin().read_line(&mut string).unwrap();
        input = string.chars().rev().map(|x| x as u8).collect();
    }

    let start = SystemTime::now();
    let mut out: Vec<char> = vec![];
    let mut iter = 0;
    while idx < tokens.len() && iter < MAX_ITER {
        match tokens[idx] {
            BfToken::RIG =>
                if pointer != stack.len() - 1 {pointer += 1}
                else {stack.push(0); pointer += 1},
            BfToken::LEF =>
                if pointer != 0 {pointer -= 1}
                else {stack.insert(0, 0)},
            BfToken::INC => {
                if stack[pointer] != u8::MAX {stack[pointer] += 1}
                else { stack[pointer] = 0 }
            }
            BfToken::DEC => {
                if stack[pointer] != 0 {stack[pointer] -= 1}
                else { stack[pointer] = 255 }
            }
            BfToken::JUM =>
                if stack[pointer] == 0 {idx = *jumps.get(&idx).unwrap()},
            BfToken::BAC =>
                if stack[pointer] != 0 {idx = *jumps.get(&idx).unwrap()}
            BfToken::ACC => stack[pointer] = input.pop().unwrap_or(0),
            BfToken::OUT => out.push(stack[pointer] as char),
            _ => ()
        }
        idx += 1;
        iter += 1;
    }

    println!("{:?}", stack);
    println!("{}", out.iter().collect::<String>());
    println!("Time taken: {:?}", SystemTime::now().duration_since(start).unwrap())
}

fn main() {
    let code = std::fs::read_to_string("code.txt").unwrap();
    parse(&code);
}
