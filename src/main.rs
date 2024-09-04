use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
enum OpCode {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart,
    LoopEnd,
}

#[derive(Clone)]
enum Instruction {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

// translate the brainfuck code into opcodes
fn trans(source: String) -> Vec<OpCode> {
    // TODO: Rewrite in map
    let mut ops = Vec::new();

    for cmd in source.chars() {
        let op = match cmd {
            '>' => Some(OpCode::IncrementPtr),
            '<' => Some(OpCode::DecrementPtr),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopStart),
            ']' => Some(OpCode::LoopEnd),
            _ => None,
        };

        match op {
            Some(op) => ops.push(op),
            None => (),
        }
    }
    ops
}

// parse the opcodes into instruction
fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut code: Vec<Instruction> = Vec::new();
    let mut loop_begin = 0;
    let mut loop_stack = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match op {
                OpCode::IncrementPtr => Some(Instruction::IncrementPtr),
                OpCode::DecrementPtr => Some(Instruction::DecrementPtr),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Read => Some(Instruction::Read),
                OpCode::Write => Some(Instruction::Write),

                OpCode::LoopStart => {
                    loop_begin = i;
                    loop_stack += 1;
                    None
                }

                OpCode::LoopEnd => {
                    panic!("OpCode::LoopEnd at {} doesn't have a OpCode::LoopStart.", i);
                }
            };

            match instruction {
                Some(instruction) => code.push(instruction),
                None => (),
            };
        } else {
            match op {
                OpCode::LoopStart => {
                    loop_stack += 1;
                }
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        code.push(Instruction::Loop(parse(
                            opcodes[loop_begin + 1..i].to_vec(),
                        )));
                    }
                }
                _ => (),
            };
        }
    }

    if loop_stack != 0 {
        panic!(
            "OpCode::LoopEnd at {} doesn't have a OpCode::LoopStart.",
            loop_begin
        );
    }
    code
}

// executes the brainfuck interpreter
fn brainfuck(instructions: &Vec<Instruction>, buffer: &mut Vec<u8>, data_ptr: &mut usize) {
    for item in instructions {
        match item {
            Instruction::IncrementPtr => *data_ptr += 1,
            Instruction::DecrementPtr => *data_ptr -= 1,
            Instruction::Increment => buffer[*data_ptr] += 1,
            Instruction::Decrement => buffer[*data_ptr] -= 1,
            Instruction::Write => print!("{}", buffer[*data_ptr] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to read stdin.");
                buffer[*data_ptr] = input[0];
            },
            Instruction::Loop(nested_loops) => {
                while buffer[*data_ptr] != 0 {
                    brainfuck(&nested_loops, buffer, data_ptr);
                }
            }
        }
    }
}

fn main() {
    // TODO: use clap to rewrite the cmdline args parsing part
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: include a .bf file");
        return();
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Failed to read from file");

    let opcodes = trans(src);
    let instructions = parse(opcodes);

    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut data_ptr = 512;

    brainfuck(&instructions, &mut buffer, & mut data_ptr);
}
