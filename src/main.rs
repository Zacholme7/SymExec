mod handlers;
mod opcodes;
mod sym_stack;
use anyhow::Result;
use std::env;
use std::fs;

use crate::handlers::*;
use crate::opcodes::*;
use crate::sym_stack::{Constant, Expr, Kind, SymVal, Term, Variable};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need a runtime binary");
    }

    // Read in the runtime binary
    let runtime_string = fs::read_to_string(&args[1])?
        .trim_start_matches("0x")
        .trim()
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

        // No loops
        if search_path(&context.path, &context.pc) {
            return context.counter;
        }

        context.path.push(context.pc);

        // Extract the handler for this opcode
        let handler = &handlers[opcode as usize];

        // We dont create symbolic values for push, dup, swap.
        if (PUSH1..=SWAP16).contains(&opcode) {
            (handler.handler)(handler, &mut context, &mut []);
            continue;
        }

        // The other opcodes have either 0 or 1 output values
        let mut sym_args: Vec<Term> = Vec::new();
        let mut sym_op: Vec<Term> = Vec::new();

        for i in 0..handler.in_args {
            sym_args.push(
                context.sym_stack.values[context.sym_stack.free_top - (i as usize) - 1].clone(),
            );
        }
        for _ in 0..handler.out_args {
            let tmp = Term {
                sym_val: SymVal {
                    value: opcode as u64,
                    kind: Kind::Symbolic,
                },
                args: sym_args.clone(),
            };
            sym_op.push(tmp);
        }

        let prev_pc = context.pc;
        (handler.handler)(handler, &mut context, &mut sym_op);

        if opcode == JUMPI {
            let mut new_constraints: Vec<Expr> = Vec::new();

            let condition_opcode: u8 = sym_args[1].sym_val.value as u8;

            if is_relational(condition_opcode) || condition_opcode == ISZERO {
                // at a branching point, convert to expression
                let expression = term_to_expression(sym_args[1].clone());
            } else {
                new_constraints = context.constraints.clone();
            }

            context.pc = prev_pc + 1;
        } else if [RETURN, REVERT, INVALID, STOP, SELFDESTRUCT].contains(&opcode) {
            return context.counter;
        }
    }
    context.counter
}

// Encode symbolic terms into DL expressions, when possible
// For now, handle only LT and GT
fn term_to_expression(term: Term) -> Vec<Expr> {
    let opcode: u8 = term.sym_val.value as u8;
    let args = term.args;

    if opcode == LT {
        if is_symbolic(&args[0]) && is_symbolic(&args[1]) {
            return vec![Expr {
                a: Variable(args[0].sym_val.value),
                b: Variable(args[1].sym_val.value),
                k: Constant(-1),
            }];
        } else if is_symbolic(&args[0]) && is_concrete(&args[1]) {
            return vec![Expr {
                a: Variable(args[0].sym_val.value),
                b: Variable(0),
                k: Constant((args[1].sym_val.value - 1) as i64),
            }];
        } else if is_concrete(&args[0]) && is_symbolic(&args[1]) {
            return vec![Expr {
                a: Variable(0),
                b: Variable(args[1].sym_val.value),
                k: Constant(-((args[1].sym_val.value + 1) as i64)),
            }];
        } else {
            panic!("Constant LT");
        }
    }

    if opcode == GT {
        let swapped_term = Term {
            sym_val: SymVal {
                value: LT as u64,
                kind: Kind::Symbolic,
            },
            args: vec![args[1].clone(), args[0].clone()],
        };
        return term_to_expression(swapped_term);
    }

    if opcode == EQ {
        return vec![
            Expr {
                a: Variable(args[0].sym_val.value),
                b: Variable(args[1].sym_val.value),
                k: Constant(0),
            },
            Expr {
                a: Variable(args[1].sym_val.value),
                b: Variable(args[0].sym_val.value),
                k: Constant(0),
            },
        ];
    }

    if opcode == ISZERO && is_symbolic(&args[0]) {
        let arg_opcode: u8 = args[0].sym_val.value as u8;
        if arg_opcode == LT || arg_opcode == GT {
            // negate child expressions
            let child = term_to_expression(args[0].to_owned());
            let first = child[0].to_owned();
            return vec![Expr {
                a: first.b,
                b: first.a,
                k: Constant(-(first.k.0 - (1_i64))),
            }];
        }
    }

    vec![]
}

fn is_symbolic(term: &Term) -> bool {
    term.sym_val.kind == Kind::Symbolic
}

fn is_concrete(term: &Term) -> bool {
    term.sym_val.kind == Kind::Concrete
}

fn search_path(path: &[usize], pc: &usize) -> bool {
    false
}
