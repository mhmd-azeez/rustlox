use num_traits::FromPrimitive;
use num_derive::FromPrimitive; 

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
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
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
            OpCode::OpConstant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        },
        None => {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} '", name, constant);
    print_value(chunk.constants[constant]);
    println!();
    return offset + 2;
}

fn print_value(value: Value) {
    print!("{}", value);
}


fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

type Value = f64;

#[derive(Debug, PartialEq, FromPrimitive)]
enum OpCode {
    OpConstant,
    OpReturn
}

struct Chunk {
   code: Vec<u8>,
   count: u8,
   constants: Vec<Value>,
   lines: Vec<i32>,
}

impl Chunk {
    fn new() -> Chunk {
        return Chunk {
            code: vec![],
            count: 0,
            constants: vec![],
            lines: vec![]
        };
    }

    fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
        self.count += 1;
    }

    fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        return (self.constants.len() - 1) as u8;
    }
}

