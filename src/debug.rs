use num_traits::FromPrimitive;

use crate::chunk::{OpCode, Chunk, Value};

pub fn disassemble_chunk(chunk: &Chunk) {
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("  | ");
    } else {
        print!("{:<4}", chunk.lines[offset]);
    }

    let instruction: u8 = chunk.code[offset];
    
    let opcode = OpCode::from_u8(instruction);

    match opcode {
        Some(value) => match value {
            OpCode::Constant => constant_instruction("OP_CONSTANT", &chunk, offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
        },
        None => {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} '", name, constant);
    print_value(chunk.constants[constant]);
    println!();
    return offset + 2;
}

pub fn print_value(value: Value) {
    print!("{}", value);
}


pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}