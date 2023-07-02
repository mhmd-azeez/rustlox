use crate::chunk::Chunk; 
use crate::chunk::OpCode;
mod chunk;
mod debug;

fn main() {
    println!("Hello, world!");
    let mut chunk : Chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::OpReturn as u8, 123);

    disassemble_chunk(&chunk, "test chunk");
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==\n", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = debug::disassemble_instruction(chunk, offset);
    }
}