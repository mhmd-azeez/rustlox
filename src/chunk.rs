use num_derive::FromPrimitive; 

pub type Value = f64;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn
}

pub struct Chunk {
   pub code: Vec<u8>,
   pub constants: Vec<Value>,
   pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![]
        };
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        return (self.constants.len() - 1) as u8;
    }
}

