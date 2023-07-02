use crate::{chunk::{Chunk, OpCode, Value}, debug};
use num_traits::FromPrimitive;

pub struct VM {
    chunk: Chunk,
    ip: usize,
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
        };
    }

    pub fn intepret(mut self) -> InterpretResult {
        loop {
            debug::disassemble_instruction(&self.chunk, self.ip);

            let instruction = OpCode::from_u8(self.read_byte());
            match instruction {
                Some(value) => match value {
                    OpCode::OpReturn => return InterpretResult::Ok,
                    OpCode::OpConstant => {
                        let index = self.read_byte();
                        let constant = self.read_constant(index);
                        debug::print_value(constant);
                        println!();
                    }
                },
                None => return InterpretResult::RuntimeError,
            }
            
        }
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