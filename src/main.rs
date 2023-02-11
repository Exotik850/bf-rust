use std::collections::HashMap;
use std::io::Write;
use std::num::Wrapping;
use std::ops::Deref;
use std::time::SystemTime;

const MAX_ITER: u64 = 1000000000;

// The BfToken enum represents the possible operations in Brainfuck language.
#[derive(Debug, Copy, Clone)]
enum BfToken {
    INC(usize),
    DEC(usize),
    LEF(usize),
    RIG(usize),
    JUM,  // Jump if the value of the current cell is zero
    BAC,  // Jump to the matching opening bracket
    ACC,  // Accept one byte of input, storing its value in the current cell
    OUT,  // Output the value of the current cell as a character
    NAN,  // Not a valid operation
}

// Implements the PartialEq trait to compare two BfToken instances.
impl PartialEq for BfToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match against the same variant for each operation, and compare the value (if any).
            (Self::INC(_), Self::INC(_)) => true,
            (Self::DEC(_), Self::DEC(_)) => true,
            (Self::LEF(_), Self::LEF(_)) => true,
            (Self::RIG(_), Self::RIG(_)) => true,
            (Self::JUM, Self::JUM) => true,
            (Self::BAC, Self::BAC) => true,
            (Self::ACC, Self::ACC) => true,
            (Self::OUT, Self::OUT) => true,
            (Self::NAN, Self::NAN) => true,
            (_, _) => false,
        }
    }
}

// Implements the From trait to convert a character to a BfToken.
impl From<char> for BfToken {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::RIG(1),
            '<' => Self::LEF(1),
            '+' => Self::INC(1),
            '-' => Self::DEC(1),
            '.' => Self::OUT,
            ',' => Self::ACC,
            '[' => Self::JUM,
            ']' => Self::BAC,
            _ => Self::NAN,
        }
    }
}

// Implements methods for the BfToken enum.
impl BfToken {
    // Converts a string of Brainfuck code to a vector of BfToken instances and a vector of jump positions.
    fn from_source(code: &str) -> (Vec<Self>, Vec<usize>) {
        let start = SystemTime::now();
        // Filter out invalid operations and collect the remaining ones in a vector.
        let mut tokens: Vec<BfToken> = code
            .chars()
            .filter_map(|c| match BfToken::from(c) {
                BfToken::NAN => None,
                T => Some(T),
            })
            .collect();
        // Combine successive instances of the same operation into a single instance with the sum of their values.
        tokens = tokens.iter().fold(vec![BfToken::NAN], |mut acc, next| {
            let last = acc.len() - 1;
            match (acc.last().unwrap(), next) {
                (BfToken::INC(N), BfToken::INC(M)) => acc[last] = BfToken::INC(N + M),
                (BfToken::LEF(N), BfToken::LEF(M)) => acc[last] = BfToken::LEF(N + M),
                (BfToken::RIG(N), BfToken::RIG(M)) => acc[last] = BfToken::RIG(N + M),
                (BfToken::DEC(N), BfToken::DEC(M)) => acc[last] = BfToken::DEC(N + M),
                (_, &curr) => acc.push(curr),
            }
            acc
        });

        // Create a map of the jumps for the bracket commands
        let mut jumps = vec![0;code.len()];
        let mut queue = vec![];
        for (idx, token) in tokens.iter().enumerate() {
            match token {
                BfToken::JUM => queue.push(idx),
                BfToken::BAC => {
                    let temp = queue
                        .pop()
                        .expect(&format!("Unopened bracket at {idx}"));

                    // Write the jump destination to the index of the token
                    jumps[temp] = idx;
                    jumps[idx] = temp;
                }
                _ => (),
            }
        }

        if !queue.is_empty() {
            panic!("Unclosed brackets at {queue:?}")
        }

        println!(
            "Compilation time: {:?}",
            SystemTime::now().duration_since(start).unwrap()
        );

        (tokens, jumps)
    }
}

fn parse(code: &str) {
    // Initialize "system" variables
    let mut stack = vec![0u8];
    let (tokens, jumps) = BfToken::from_source(code);
    let mut pointer = 0;
    let mut idx = 0;

    // If there is an input token, convert an input string to it's bytes
    let mut input: Vec<u8> = vec![];
    if tokens.contains(&BfToken::ACC) {
        println!("Enter the input string for the code: ");
        let mut string = "".to_string();
        std::io::stdin().read_line(&mut string).unwrap();
        if string.chars().all(|p| p.is_numeric()) {
            input = vec![string.parse::<u8>().unwrap()]
        } else {
            input = string.chars().rev().map(|x| x as u8).collect();
        }
    }

    let start = SystemTime::now();
    let mut out: Vec<char> = vec![];
    let mut iter = 0u64; // For optional iteration cap
    while idx < tokens.len() {
        match tokens[idx] {
            BfToken::RIG(N) => {
                // Check if there is room on the stack to move right, if not make room
                if pointer + N >= stack.len() {
                    stack.extend(vec![0; N]);
                }
                pointer += N;
            }
            BfToken::LEF(N) => {
                // Opposite for moving left
                if pointer >= N {
                    pointer -= N
                } else {
                    stack.splice(0..0, vec![0u8; N]);
                }
            }
            BfToken::INC(N) => stack[pointer] = stack[pointer].wrapping_add(N as u8),
            BfToken::DEC(N) => stack[pointer] = stack[pointer].wrapping_sub(N as u8),
            BfToken::JUM => if stack[pointer] == 0 { idx = jumps[idx] },
            BfToken::BAC => if stack[pointer] != 0 { idx = jumps[idx] },
            BfToken::ACC => stack[pointer] = input.pop().unwrap_or(0),
            BfToken::OUT => out.push(stack[pointer] as char),
            _ => (),
        }
        idx += 1;
        iter += 1;
    }

    println!("{:?}", stack);
    println!("{}", out.iter().collect::<String>());
    println!(
        "Time taken: {:?}\nIterations: {iter}",
        SystemTime::now().duration_since(start).unwrap()
    )
}

fn main() {
    let code = std::fs::read_to_string("code.txt").unwrap();
    parse(&code);
}
