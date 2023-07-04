use crate::{
    chunk::{Chunk, OpCode, Value},
    compiler::{compile}, debug::{self, disassemble_chunk},
};
use num_traits::FromPrimitive;

pub struct VM {
    // chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: Vec<char>) -> InterpretResult {
        let result = compile(&source);

        return match result {
            Some(chunk) => self.run(&chunk),
            None => return InterpretResult::CompileError,
        };
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {

        println!("code: ");
        disassemble_chunk(chunk);
        println!("----------------");

        loop {
            print!("          ");

            for slot in &self.stack {
                print!("[ ");
                debug::print_value(*slot);
                print!(" ]");
            }

            println!();

            debug::disassemble_instruction(chunk, self.ip);

            let instruction = OpCode::from_u8(self.read_byte(chunk));
            match instruction {
                Some(value) => match value {
                    OpCode::Constant => {
                        let index = self.read_byte(chunk);
                        let constant = chunk.read_constant(index);
                        self.push(constant);
                    }
                    OpCode::Add => self.binary_op(|a, b| a + b),
                    OpCode::Subtract => self.binary_op(|a, b| a - b),
                    OpCode::Multiply => self.binary_op(|a, b| a * b),
                    OpCode::Divide => self.binary_op(|a, b| a / b),
                    OpCode::Negate => {
                        let value = self.pop();
                        self.push(-value);
                    }
                    OpCode::Return => {
                        debug::print_value(self.pop());
                        println!();
                        return InterpretResult::Ok;
                    }
                },
                None => return InterpretResult::RuntimeError,
            }
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte: u8 = chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}
