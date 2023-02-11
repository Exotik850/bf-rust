use std::collections::HashMap;
use std::io::Write;
use std::num::Wrapping;
use std::ops::Deref;
use std::time::SystemTime;

const MAX_ITER: u32 = 1000000;

#[derive(Debug, Copy, Clone)]
enum BfToken {
    INC(usize), DEC(usize), LEF(usize), RIG(usize), JUM, BAC, ACC, OUT, NAN
}

impl PartialEq for BfToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::INC(N), Self::INC(A)) => true,
            (Self::DEC(N), Self::DEC(A)) => true,
            (Self::LEF(N), Self::LEF(A)) => true,
            (Self::RIG(N), Self::RIG(A)) => true,
            (Self::JUM, Self::JUM) => true,
            (Self::BAC, Self::BAC) => true,
            (Self::ACC, Self::ACC) => true,
            (Self::OUT, Self::OUT) => true,
            (Self::NAN, Self::NAN) => true,
            (_, _) => false
        }
    }
}

impl From<char> for BfToken {
    fn from(value: char) -> Self {
        match value {
            '>'=>Self::RIG(1),
            '<'=>Self::LEF(1),
            '+'=>Self::INC(1),
            '-'=>Self::DEC(1),
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
        let mut tokens: Vec<BfToken> = code.chars().filter_map(|c| match BfToken::from(c) {BfToken::NAN=>None, T=>Some(T)}).collect();
        tokens = tokens.iter().fold(vec![BfToken::NAN], |mut acc, next| {
            let last = acc.len()-1;
            match (acc.last().unwrap(), next) {
                (BfToken::INC(N), BfToken::INC(M)) => {acc[last] = BfToken::INC(N + M)},
                (BfToken::LEF(N), BfToken::LEF(M)) => {acc[last] = BfToken::LEF(N + M)},
                (BfToken::RIG(N), BfToken::RIG(M)) => {acc[last] = BfToken::RIG(N + M)},
                (BfToken::DEC(N), BfToken::DEC(M)) => {acc[last] = BfToken::DEC(N + M)},
                (prev, &curr) => {acc.push(curr)}
            }
            acc
        });
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
    let mut stack = vec![0u8];
    let (tokens, jumps) = BfToken::from_source(code);
    let mut pointer = 0;
    let mut idx = 0;

    let mut input: Vec<u8> = vec![];
    if tokens.contains(&BfToken::ACC) {
        println!("Enter the input string for the code: ");
        let mut string = "".to_string();
        std::io::stdin().read_line(&mut string).unwrap();
        if string.chars().all(|p| p.is_numeric()) {input = vec!(string.parse::<u8>().unwrap())}
        else { input = string.chars().rev().map(|x| x as u8).collect(); }
    }

    let start = SystemTime::now();
    let mut out: Vec<char> = vec![];
    let mut iter = 0i64;
    while idx < tokens.len() {
        match tokens[idx] {
            BfToken::RIG(N) => if pointer + N >= stack.len() {stack.extend(vec![0; N]); pointer += N}
                                   else {pointer += N},
            BfToken::LEF(N) => if pointer - N >= 0 {pointer -= N} else {stack.splice(0..0, vec![0u8;N]);},
            BfToken::INC(N) => stack[pointer] = stack[pointer].wrapping_add(N as u8),
            BfToken::DEC(N) => stack[pointer] = stack[pointer].wrapping_sub(N as u8),
            BfToken::JUM => if stack[pointer] == 0 {idx = *jumps.get(&idx).unwrap()},
            BfToken::BAC => if stack[pointer] != 0 {idx = *jumps.get(&idx).unwrap()}
            BfToken::ACC => stack[pointer] = input.pop().unwrap_or(0),
            BfToken::OUT => out.push(stack[pointer] as char),
            _ => ()
        }
        idx += 1;
        iter += 1;
    }

    println!("{:?}", stack);
    println!("{}", out.iter().collect::<String>());
    println!("Time taken: {:?}\nIterations: {iter}", SystemTime::now().duration_since(start).unwrap())
}

fn main() {
    let code = std::fs::read_to_string("code.txt").unwrap();
    parse(&code);
}
