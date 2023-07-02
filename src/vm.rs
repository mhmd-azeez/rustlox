use crate::{chunk::{Chunk, OpCode, Value}, debug};
use num_traits::FromPrimitive;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        return VM {
            chunk: chunk,
            ip: 0,
            stack: vec![]
        };
    }

    pub fn intepret(mut self) -> InterpretResult {
        loop {

            print!("          ");

            for slot in self.stack.iter() {
                print!("[ ");
                debug::print_value(*slot);
                print!(" ]");
            }

            println!();

            debug::disassemble_instruction(&self.chunk, self.ip);

            let instruction = OpCode::from_u8(self.read_byte());
            match instruction {
                Some(value) => match value {
                    OpCode::OpReturn => {
                        debug::print_value(self.pop());
                        println!();
                        return InterpretResult::Ok;
                    },
                    OpCode::OpConstant => {
                        let index = self.read_byte();
                        let constant = self.read_constant(index);
                        self.push(constant);
                    }
                },
                None => return InterpretResult::RuntimeError,
            }
            
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        return self.stack.pop().unwrap();
    }

    fn read_byte(&mut self) -> u8 {
        let byte: u8 = self.chunk.code[self.ip];
        self.ip += 1;
        return byte;
    }

    fn read_constant(&self, index: u8) -> Value {
        return self.chunk.constants[index as usize];
    }
}