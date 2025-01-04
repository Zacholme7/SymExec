mod opcodes;
mod handlers;
mod sym_stack;
use std::fs;
use std::env;
use anyhow::Result;

use crate::handlers::*;
use crate::opcodes::*;
use crate::sym_stack::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }

    let runtime_string = fs::read_to_string(&args[1])?
        .trim_start_matches("0x").to_string();
    let runtime = hex::decode(runtime_string)?;

    run(runtime);
    Ok(())

}

fn run(runtime: Vec<u8>) -> u64 {
    let mut context = EvmContext {
        counter: 1,
        code: runtime,
        ..Default::default()
    };

    // Interpret the runtime bytecode
    while context.pc < context.code.len() {
        let opcode = context.code[context.pc];

        if search_path(&context.path, &context.pc) {
            return context.counter;
        }
        context.path.push(context.pc);


        // dont create symbolic values for push, dup, swap
        if PUSH1 <= opcode && opcode <= SWAP16 {
            let mut n = 0;
            if PUSH1 <= opcode && opcode <= PUSH32 {
                n = opcode - PUSH1 + 1;
            } else if DUP1 <= opcode && opcode <= DUP1 {
                n = opcode - DUP1 + 1;
            } else if SWAP1 <= opcode && opcode <= SWAP16 {
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
