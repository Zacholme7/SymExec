mod handlers;
mod opcodes;
mod sym_stack;
use anyhow::Result;
use std::env;
use std::fs;

use crate::handlers::*;
use crate::opcodes::*;
use crate::sym_stack::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }

    let runtime_string = fs::read_to_string(&args[1])?
        .trim_start_matches("0x")
        .to_string();
    let runtime = hex::decode(runtime_string)?;

    run(runtime);
    Ok(())
}

fn run(runtime: Vec<u8>) -> u64 {
    // Create the Symbolic Evm Context
    let mut context = EvmContext {
        counter: 1,
        code: runtime,
        ..Default::default()
    };

    // Create all of the handlers
    let handlers = sym_handlers();

    // Interpret the runtime bytecode
    while context.pc < context.code.len() {
        let opcode = context.code[context.pc];

        if search_path(&context.path, &context.pc) {
            return context.counter;
        }
        context.path.push(context.pc);

        let handler = &handlers[opcode as usize];

        // dont create symbolic values for push, dup, swap
        if (PUSH1..=SWAP16).contains(&opcode) {
            let mut n = 0;
            if (PUSH1..=PUSH32).contains(&opcode) {
                n = opcode - PUSH1 + 1;
            } else if (DUP1..=DUP16).contains(&opcode) {
                n = opcode - DUP1 + 1;
            } else if (SWAP1..=SWAP16).contains(&opcode) {
                n = opcode - SWAP1 + 1;
            }
            // do the handler
            continue;
        }

        // the other opcodes have either 0 or 1 output values
    }
    context.counter
}

fn search_path(path: &[usize], pc: &usize) -> bool {
    todo!()
}
