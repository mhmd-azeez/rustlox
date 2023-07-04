use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
use vm::InterpretResult;

use crate::vm::VM;
mod chunk;
mod debug;
mod vm;
mod compiler;
mod scanner;

fn main() -> Result<(), i32> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => return repl(),
        2 => return run_file(&args[1]),
        _ => {
            println!("Usage: rustlox [path]");
            return Err(64);
        }
    }
}

fn repl() -> Result<(), i32> {
    let mut vm = VM::new();

    loop {
        let result = read_line();
        match result {
            Ok(line) => {
                if line.len() == 0 {
                    println!("bye!");
                    break;
                } else {
                    interpret(&mut vm, line);
                }
            }
            Err(err) => {
                println!("Invalid input: {}", err);
                break;
            }
        }
    }

    return Ok(());
}

fn run_file(path: &str) -> Result<(), i32> {
    let mut vm = VM::new();

    let result = read_file(path);
    match result {
        Ok(line) => {
            let result = interpret(&mut vm, line);
            match result {
                InterpretResult::CompileError => return Err(65),
                InterpretResult::RuntimeError => return Err(70),
                InterpretResult::Ok => return Ok(()),
            };
        }
        Err(err) => {
            println!("Could not open file '{}': {}", path, err);
            return Err(74);
        }
    };
}

fn interpret(vm: &mut VM, source: Vec<char>) -> InterpretResult {
   return vm.interpret(source);
}

fn read_line() -> Result<Vec<char>, io::Error> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer)?;
    Ok(buffer.trim().chars().collect())
}

fn read_file(file_path: &str) -> Result<Vec<char>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.chars().collect())
}
