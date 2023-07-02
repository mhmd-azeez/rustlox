use crate::chunk::Chunk; 
use crate::chunk::OpCode;
use crate::vm::VM;
mod chunk;
mod debug;
mod vm;

fn main() {
    println!("Hello, world!");
    let mut chunk : Chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::OpAdd as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::OpDivide as u8, 123);

    chunk.write(OpCode::OpNegate as u8, 123);

    chunk.write(OpCode::OpReturn as u8, 123);

    let vm = VM::new(chunk);
    vm.intepret();
}