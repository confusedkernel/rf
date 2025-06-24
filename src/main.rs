use clap::Parser;
use std::fs::File;
use std::io::{ErrorKind, Read};

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

#[derive(Parser, Debug)]
struct CmdArgs {
    file: String,
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
            Instruction::Increment => buffer[*data_ptr] = buffer[*data_ptr].wrapping_add(1),
            Instruction::Decrement => buffer[*data_ptr] = buffer[*data_ptr].wrapping_sub(1),
            Instruction::Write => print!("{}", buffer[*data_ptr] as char),
            Instruction::Read => {
                // Do nothing if input has ended
                //
                // <https://brainfuck.org/brainfuck.html>
                // > However, if the input is coming from a file and all the bytes from
                // > that file have been received already, or if the user signals an analogous
                // > end-of-input condition from the keyboard, a request for input will not pause
                // > the program; instead, the input command will have no effect and the program
                // > will go on running. At least, that's the behavior of Urban Müller's compiler and
                // > three-line C interpreter; some other implementations also set the cell indicated
                // > by the pointer to -1, 0, or 255. Several reasons for preferring the "no change"
                // > behavior are mentioned here.
                let mut input: [u8; 1] = [0; 1];
                match std::io::stdin().read_exact(&mut input) {
                    Err(err) if err.kind() == ErrorKind::UnexpectedEof => continue,
                    _ => buffer[*data_ptr] = input[0],
                }
            }
            Instruction::Loop(nested_loops) => {
                while buffer[*data_ptr] != 0 {
                    brainfuck(&nested_loops, buffer, data_ptr);
                }
            }
        }
    }
}

fn main() {
    let args = CmdArgs::parse();

    let filename = args.file;

    let mut file = File::open(filename).expect("file not found");
    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect("Failed to read from file");

    let opcodes = trans(src);
    let instructions = parse(opcodes);

    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut data_ptr = 512;

    brainfuck(&instructions, &mut buffer, &mut data_ptr);
}
