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

    chunk.write(OpCode::OpReturn as u8, 123);

    let vm = VM::new(chunk);
    vm.intepret();
}