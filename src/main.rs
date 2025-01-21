use std::io::Read;
use std::ops::AddAssign;
use std::time::{Instant, SystemTime};

const MAX_ITER: u64 = 1000000000;

// Represents the possible operations in Brainf*** language.
#[derive(Debug, Copy, Clone)]
enum BfToken {
    CEL(isize), // Increment the current cell by N
    MOV(isize), // Move the pointer by N
    JUM,        // Jump if the value of the current cell is zero
    BAC,        // Jump to the matching opening bracket
    ACC,        // Accept one byte of input, storing its value in the current cell
    OUT,        // Output the value of the current cell as a character
    NAN,        // Not a valid operation
}

impl PartialEq for BfToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match against the same variant for each operation
            (Self::CEL(_), Self::CEL(_)) => true,
            (Self::MOV(_), Self::MOV(_)) => true,
            (_, _) => false,
        }
    }
}

// Implements the From trait to convert a character to a BfToken.
impl From<char> for BfToken {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::MOV(1),
            '<' => Self::MOV(-1),
            '+' => Self::CEL(1),
            '-' => Self::CEL(-1),
            '.' => Self::OUT,
            ',' => Self::ACC,
            '[' => Self::JUM,
            ']' => Self::BAC,
            _ => Self::NAN,
        }
    }
}

impl From<BfToken> for String {
    fn from(value: BfToken) -> Self {
        match value {
            BfToken::CEL(n) => {
                if n > 0 {
                    "+".repeat(n as usize)
                } else if n < 0 {
                    "-".repeat(n as usize)
                } else {
                    "".to_string()
                }
            }
            BfToken::MOV(n) => {
                if n > 0 {
                    ">".repeat(n as usize)
                } else if n < 0 {
                    "<".repeat(n as usize)
                } else {
                    "".to_string()
                }
            }
            BfToken::JUM => {"[".to_string()}
            BfToken::BAC => {"]".to_string()}
            BfToken::ACC => {",".to_string()}
            BfToken::OUT => {".".to_string()}
            BfToken::NAN => {"".to_string()}
        }
    }
}

impl AddAssign for BfToken {
    fn add_assign(&mut self, rhs: Self) {
        match (self, &rhs) {
            (Self::CEL(N), Self::CEL(A)) => *N += A,
            (Self::MOV(N), Self::MOV(A)) => *N += A,
            (_, _) => (),
        }
    }
}

impl BfToken {
    fn find_jumps(tokens: &Vec<Self>) -> Vec<usize> {
        // Create a map of the jumps for the bracket commands
        let mut jumps = vec![0; tokens.len()];
        let mut queue = vec![];
        for (idx, token) in tokens.iter().enumerate() {
            match token {
                BfToken::JUM => queue.push(idx),
                BfToken::BAC => {
                    let temp = queue
                        .pop()
                        .unwrap_or_else(|| panic!("Unopened bracket at {idx}"));

                    // Write the jump destination to the index of the token
                    jumps[temp] = idx;
                    jumps[idx] = temp;
                }
                _ => (),
            }
        }

        assert!(queue.is_empty(), "Unclosed bracket");

        jumps
    }

    // Converts a string of Brainfuck code to a vector of BfToken instances and a vector of jump positions.
    fn from_source(code: &str) -> (Vec<Self>, Vec<usize>) {
        let start = Instant::now();
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
            if acc[last].eq(next) {
                acc[last] += *next;
            } else {
                acc.push(*next);
            }
            acc
        });

        let jumps = Self::find_jumps(&tokens);

        println!("Compilation time: {:?}", start.elapsed());

        (tokens, jumps)
    }
}

fn parse(code: &str) {
    // Initialize "system" variables
    let mut stack = vec![0u8];
    let (tokens, jumps) = BfToken::from_source(code);
    let mut pointer = 0usize;
    let mut idx = 0;

    // If there is an input token, convert an input string to it's bytes
    let mut input: Vec<u8> = vec![];
    if tokens.contains(&BfToken::ACC) {
        println!("Enter the input string for the code: ");
        std::io::stdin().read_to_end(&mut input).unwrap();
        input.reverse();
    }

    let start = Instant::now();
    let mut out: Vec<char> = vec![];
    let mut iter = 0u64; // For optional iteration cap
    while idx < tokens.len() {
        match tokens[idx] {
            BfToken::MOV(N) => {
                if N > 0 {
                    let n = N as usize;
                    // Check if there is room on the stack to move right, if not make room
                    if pointer + n >= stack.len() {
                        stack.extend(std::iter::repeat_n(0, n));
                    }
                    pointer += n;
                } else {
                    // Opposite for moving left
                    let n = N.unsigned_abs();
                    if pointer >= n {
                        pointer -= n;
                    } else {
                        stack.splice(0..0, std::iter::repeat_n(0, n - pointer));
                    }
                }
            }
            BfToken::CEL(n) => {
                if n > 0 {
                    stack[pointer] = stack[pointer].wrapping_add(n as u8);
                } else {
                    stack[pointer] = stack[pointer].wrapping_sub((-n) as u8);
                }
            }
            BfToken::JUM => {
                if stack[pointer] == 0 {
                    idx = jumps[idx];
                }
            }
            BfToken::BAC => {
                if stack[pointer] != 0 {
                    idx = jumps[idx];
                }
            }
            BfToken::ACC => stack[pointer] = input.pop().unwrap_or(0),
            BfToken::OUT => out.push(stack[pointer] as char),
            _ => (),
        }
        idx += 1;
        iter += 1;
    }
    let time = start.elapsed();
    println!("{stack:?}");
    println!("{}", out.iter().collect::<String>());
    println!("Time taken: {time:?}\nCommands Processed: {iter}");
    // println!("{tokens:?}");
}

fn main() {
    let code = std::fs::read_to_string("code.txt").unwrap();
    parse(&code);
}
