

use crate::sym_stack::{EvmSymStack, Expr};
use crate::Term;

#[derive(Default)]
pub struct EvmContext {
    pub code: Vec<u8>,
    pub sym_stack: EvmSymStack,
    pub pc: usize,
    pub path: Vec<usize>,
    pub constraints: Vec<Expr>,
    pub counter: u64,
}


pub struct OpcodeHandler {
    handler: fn(&OpcodeHandler, &mut EvmContext, &mut Vec<Term>),
    in_args: u8,
    out_args: u8
}

pub enum HandlerType {
    Unimplemented,
    Base,
    Push,
    Dup,
    Swap,
    Jump,
    JumpI,
    Terminating
}

impl OpcodeHandler { 
    pub fn new(handler_type: HandlerType, in_args: u8, out_args: u8) -> Self {
        let handler = match handler_type {
            HandlerType::Unimplemented => Self::handle_unimplemented,
            HandlerType::Base => Self::handle_base,
            HandlerType::Push => Self::handle_push,
            HandlerType::Dup => Self::handle_dup,
            HandlerType::Swap => Self::handle_swap,
            HandlerType::Jump => Self::handle_jump,
            HandlerType::JumpI => Self::handle_jumpi,
            HandlerType::Terminating => Self::handle_terminating,
        };
        Self { handler, in_args, out_args }
    }

    fn handle_unimplemented(&self, context: &mut EvmContext, sym_vals: &mut Vec<Term>) {
        self.handle_base(context, sym_vals);
        context.pc += 1 ;
    }

    fn handle_base(&self, context: &mut EvmContext, sym_vals: &mut Vec<Term>) {


    }

    fn handle_push(&self, context: &mut EvmContext, sym_vals: &mut Vec<Term>) {
        todo!()
    }

    fn handle_dup(&self, context: &mut EvmContext,  sym_vals: &mut Vec<Term>) {
        todo!()
    }

    fn handle_swap(&self, context: &mut EvmContext,  sym_vals: &mut Vec<Term>) {
        todo!()
    }

    fn handle_jump(&self, context: &mut EvmContext,  sym_vals: &mut Vec<Term>) {
        todo!()
    }

    fn handle_jumpi(&self, context: &mut EvmContext, sym_vals: &mut Vec<Term>) {
        todo!()
    }

    fn handle_terminating(&self, context: &mut EvmContext, sym_vals: &mut Vec<Term>) {
        self.handle_base(context, sym_vals)
    }


}




// Register jumptable of handlers for each opcode
fn sym_handlers() -> [OpcodeHandler; 256] {
    [
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0),

        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
        OpcodeHandler::new(HandlerType::Terminating, 0, 0),
    ]

}


