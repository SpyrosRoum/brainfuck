use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{env, io};

const MEMORY_SIZE: usize = 30_000;

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Print,
    Read,
    StartLoop(usize),
    EndLoop(usize),
}

impl From<(char, usize)> for Instruction {
    fn from((c, i): (char, usize)) -> Self {
        match c {
            '[' => Self::StartLoop(i),
            ']' => Self::EndLoop(i),
            _ => panic!("Char `{}` is not a valid instruction", c),
        }
    }
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Increment,
            '-' => Self::Decrement,
            '>' => Self::MoveRight,
            '<' => Self::MoveLeft,
            '.' => Self::Print,
            ',' => Self::Read,
            _ => panic!("Char `{}` is not a valid instruction", c),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args
        .get(1)
        .unwrap_or_else(|| panic!("I need a file to work with"));

    let file = File::open(file_name).expect("Could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut src = String::new();
    buf_reader
        .read_to_string(&mut src)
        .expect("Could not read file");

    let instructions = parse(&src);
    execute(instructions);
}

fn parse(src: &str) -> Vec<Instruction> {
    src.chars()
        .filter(|c| "<>+-.,[]".contains(*c))
        .enumerate()
        .map(|(i, c)| match c {
            '[' => {
                let index = find_index(src, '[', ']', i, src.len()-1);
                Instruction::from((c, index))
            }
            ']' => {
                let index = find_index(src, ']', '[', i, 0);
                Instruction::from((c, index))
            }
            _ => Instruction::from(c),
        })
        .collect()
}

fn find_index(
    src: &str,
    start_symbol: char,
    close_symbol: char,
    start_index: usize,
    stop_index: usize,
) -> usize {
    let mut index = start_index;
    let mut opened = 0;

    loop {
        let cur = src.chars().filter(|c| "<>+-.,[]".contains(*c)).nth(index).unwrap();
        if cur == start_symbol {
            opened += 1;
        } else if cur == close_symbol {
            opened -= 1;
        }

        if opened == 0 {
            return index;
        }
        if index == stop_index {
            panic!("Unmatched loop");
        } else if start_index < stop_index {
            index += 1;
        } else {
            index -= 1;
        }
    }
}

fn execute(instructions: Vec<Instruction>) {
    let mut memory = [0u8; MEMORY_SIZE];
    let mut mem_pointer = 0;
    let mut pc = 0;
    let mut stdin = io::stdin();

    while let Some(instruction) = instructions.get(pc) {
        match *instruction {
            Instruction::MoveRight => mem_pointer += 1,
            Instruction::MoveLeft => mem_pointer -= 1,
            Instruction::Increment => memory[mem_pointer] += 1,
            Instruction::Decrement => memory[mem_pointer] -= 1,
            Instruction::Print => {
                print!("{}", memory[mem_pointer] as char);
                io::stdout().flush().unwrap();
            }
            Instruction::Read => {
                let mut buf = [0u8; 1];
                stdin.read_exact(&mut buf).unwrap();
                memory[mem_pointer] = buf[0];
            }
            Instruction::StartLoop(i) => {
                if memory[mem_pointer] == 0 {
                    pc = i;
                }
            }
            Instruction::EndLoop(i) => {
                if memory[mem_pointer] != 0 {
                    pc = i;
                }
            }
        }
        pc += 1;
    }
}
