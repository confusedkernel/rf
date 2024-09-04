use std::env;
use std::io::Read;
use std::fs::File;

enum OpCode {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart,
    LoopEnd
}

enum Instruction {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

fn trans(source: String) -> Vec<OpCode> {
    let mut ops = Vec::new();

    for cmd in source.chars() {
        let ops = match cmd {
            '>' => Some(OpCode::IncrementPtr),
            '<' => Some(OpCode::DecrementPtr),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopStart),
            ']' => Some(OpCode::LoopEnd),
            _ => None
        };
    }
}
