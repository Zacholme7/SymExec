use crate::sym_stack::{EvmSymStack, Expr, Kind, SymVal, Term};

#[derive(Default, Debug)]
pub struct EvmContext {
    pub code: Vec<u8>,
    pub sym_stack: EvmSymStack,
    pub pc: usize,
    pub path: Vec<usize>,
    pub constraints: Vec<Expr>,
    pub counter: u64,
}

#[derive(Debug, Clone)]
pub struct OpcodeHandler {
    pub handler: fn(&OpcodeHandler, &mut EvmContext, &mut [Term]),
    pub in_args: u8,
    pub out_args: u8,
    bytecode_reads: u8,
}

pub enum HandlerType {
    Unimplemented,
    Base,
    Push,
    Dup,
    Swap,
    Jump,
    JumpI,
    Terminating,
}

impl OpcodeHandler {
    pub fn new(handler_type: HandlerType, in_args: u8, out_args: u8, bytecode_reads: u8) -> Self {
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
        Self {
            handler,
            in_args,
            out_args,
            bytecode_reads,
        }
    }

    fn handle_base(&self, context: &mut EvmContext, sym_vals: &mut [Term]) {
        // pop in_args num items off of the stack
        for _ in 0..self.in_args {
            context.sym_stack.sym_pop();
        }

        // push out_args num items onto the stack
        for i in 0..self.out_args {
            context.sym_stack.sym_push(sym_vals[i as usize].clone());
        }
    }

    fn handle_unimplemented(&self, context: &mut EvmContext, sym_vals: &mut [Term]) {
        self.handle_base(context, sym_vals);
        context.pc += 1;
    }

    fn handle_terminating(&self, context: &mut EvmContext, sym_vals: &mut [Term]) {
        self.handle_base(context, sym_vals)
    }

    fn handle_dup(&self, context: &mut EvmContext, _: &mut [Term]) {
        context.sym_stack.sym_dup(self.in_args);
        context.pc += 1;
    }

    fn handle_swap(&self, context: &mut EvmContext, _: &mut [Term]) {
        context.sym_stack.sym_swap(self.in_args as usize);
        context.pc += 1;
    }

    fn handle_push(&self, context: &mut EvmContext, _: &mut [Term]) {
        let mut word: u64 = 0;
        for i in 0..self.bytecode_reads {
            word |= (context.code[context.pc + i as usize + 1]) as u64;
            if i < self.bytecode_reads - 1 {
                word <<= 8;
            }
        }

        let concrete_var = Term {
            sym_val: SymVal {
                value: word,
                kind: Kind::Concrete,
            },
            args: Vec::new(),
        };

        context.sym_stack.sym_push(concrete_var);
        context.pc += self.bytecode_reads as usize + 1;
    }

    fn handle_jump(&self, context: &mut EvmContext, sym_vals: &mut [Term]) {
        let top = context.sym_stack.sym_top();
        if top.sym_val.kind == Kind::Symbolic {
            panic!("Symbolic Jump");
        }

        self.handle_base(context, sym_vals);
        context.pc = top.sym_val.value as usize;
    }

    fn handle_jumpi(&self, context: &mut EvmContext, sym_vals: &mut [Term]) {
        let top = context.sym_stack.sym_top();
        if top.sym_val.kind == Kind::Symbolic {
            panic!("Symbolic Jump");
        }

        self.handle_base(context, sym_vals);
        context.pc = top.sym_val.value as usize;
    }
}

// Register jumptable of handlers for each opcode
pub fn sym_handlers() -> [OpcodeHandler; 256] {
    [
        // 0x00 - 0x0F: Stop and Arithmetic Operations
        OpcodeHandler::new(HandlerType::Terminating, 0, 0, 0), // 0x00 STOP: Halts execution
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x01 ADD: a + b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x02 MUL: a * b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x03 SUB: a - b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x04 DIV: a ÷ b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x05 SDIV: a ÷ b (signed)
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x06 MOD: a % b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x07 SMOD: a % b (signed)
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 1, 0),        // 0x08 ADDMOD: (a + b) % N
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 1, 0),        // 0x09 MULMOD: (a * b) % N
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0),        // 0x0A EXP: a ^ b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x0B SIGNEXTEND: Extends length of two's complement signed integer
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x0C (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x0D (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x0E (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x0F (Invalid)
        // 0x10 - 0x1F: Comparison & Bitwise Logic Operations
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x10 LT: a < b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x11 GT: a > b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x12 SLT: a < b (signed)
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x13 SGT: a > b (signed)
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x14 EQ: a == b
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x15 ISZERO: a == 0
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x16 AND: a & b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x17 OR: a | b
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x18 XOR: a ^ b
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x19 NOT: ~a
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x1A BYTE: Get byte at index
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x1B SHL: Left shift
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x1C SHR: Right shift
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x1D SAR: Arithmetic right shift
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x1E (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x1F (Invalid)
        // 0x20: SHA3
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 1, 0), // 0x20 SHA3: Computes Keccak-256 hash
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x21 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x22 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x23 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x24 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x25 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x26 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x27 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x28 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x29 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2A (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2B (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2C (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2D (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2E (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x2F (Invalid)
        // 0x30 - 0x3F: Environmental Information
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x30 ADDRESS: Get address of currently executing accounthandler
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x31 BALANCE: Get balance of given account
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x32 ORIGIN: Get execution origination address
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x33 CALLER: Get caller address
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x34 CALLVALUE: Get deposited value by the instruction/transaction
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x35 CALLDATALOAD: Get input data of current environment
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x36 CALLDATASIZE: Get size of input data
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 0, 0), // 0x37 CALLDATACOPY: Copy input data to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x38 CODESIZE: Get size of code running in current environment
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 0, 0), // 0x39 CODECOPY: Copy code running in current environment to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x3A GASPRICE: Get price of gas in current environment
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x3B EXTCODESIZE: Get size of an account's code
        OpcodeHandler::new(HandlerType::Unimplemented, 4, 0, 0), // 0x3C EXTCODECOPY: Copy an account's code to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x3D RETURNDATASIZE: Get size of output data from previous call
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 0, 0), // 0x3E RETURNDATACOPY: Copy output data from previous call to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x3F EXTCODEHASH: Get hash of an account's code
        // 0x40 - 0x47: Block Information
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x40 BLOCKHASH: Get hash of recent block
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x41 COINBASE: Get current block's beneficiary address
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x42 TIMESTAMP: Get block's timestamp
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x43 NUMBER: Get current block number
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x44 DIFFICULTY: Get current block's difficulty
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x45 GASLIMIT: Get current block's gas limit
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x46 CHAINID: Get current chain ID
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x47 SELFBALANCE: Get balance of currently executing account
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0), // 0x48 BASEFEE: Get base fee
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x49 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4A (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4B (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4C (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4D (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4E (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x4F (Invalid)
        // 0x50 - 0x5F: Stack, Memory, Storage and Flow Operations
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 0, 0), // 0x50 POP: Remove item from stack
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x51 MLOAD: Load word from memory
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 0, 0), // 0x52 MSTORE: Save word to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 0, 0), // 0x53 MSTORE8: Save byte to memory
        OpcodeHandler::new(HandlerType::Unimplemented, 1, 1, 0), // 0x54 SLOAD: Load word from storage
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 0, 0), // 0x55 SSTORE: Save word to storage
        OpcodeHandler::new(HandlerType::Jump, 1, 0, 0), // 0x56 JUMP: Alter program counter
        OpcodeHandler::new(HandlerType::JumpI, 2, 0, 0), // 0x57 JUMPI: Conditionally alter program counter
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0),  // 0x58 PC: Get program counter
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0),  // 0x59 MSIZE: Get memory size
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 1, 0),  // 0x5A GAS: Get available gas
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x5B JUMPDEST: Mark valid jump destination
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x5C (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x5D (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x5E (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0x5F (Invalid)
        // 0x60 - 0x7F: Push Operations
        OpcodeHandler::new(HandlerType::Push, 0, 1, 1), // 0x60 PUSH1: Place 1-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 2), // 0x61 PUSH2: Place 2-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 3), // 0x62 PUSH3: Place 3-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 4), // 0x63 PUSH4: Place 4-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 5), // 0x64 PUSH5: Place 5-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 6), // 0x65 PUSH6: Place 6-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 7), // 0x66 PUSH7: Place 7-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 8), // 0x67 PUSH8: Place 8-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 9), // 0x68 PUSH9: Place 9-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 10), // 0x69 PUSH10: Place 10-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 11), // 0x6A PUSH11: Place 11-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 12), // 0x6B PUSH12: Place 12-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 13), // 0x6C PUSH13: Place 13-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 14), // 0x6D PUSH14: Place 14-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 15), // 0x6E PUSH15: Place 15-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 16), // 0x6F PUSH16: Place 16-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 17), // 0x70 PUSH17: Place 17-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 18), // 0x71 PUSH18: Place 18-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 19), // 0x72 PUSH19: Place 19-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 20), // 0x73 PUSH20: Place 20-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 21), // 0x74 PUSH21: Place 21-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 22), // 0x75 PUSH22: Place 22-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 23), // 0x76 PUSH23: Place 23-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 24), // 0x77 PUSH24: Place 24-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 25), // 0x78 PUSH25: Place 25-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 26), // 0x79 PUSH26: Place 26-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 27), // 0x7A PUSH27: Place 27-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 28), // 0x7B PUSH28: Place 28-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 29), // 0x7C PUSH29: Place 29-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 30), // 0x7D PUSH30: Place 30-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 31), // 0x7E PUSH31: Place 31-byte item on stack
        OpcodeHandler::new(HandlerType::Push, 0, 1, 32), // 0x7F PUSH32: Place 32-byte item on stack
        // 0x80 - 0x8F: Duplication Operations
        // Note: For DUP operations, in_args is how deep to copy from, out_args is in_args + 1
        OpcodeHandler::new(HandlerType::Dup, 1, 2, 0), // 0x80 DUP1: Duplicate 1st stack item
        OpcodeHandler::new(HandlerType::Dup, 2, 3, 0), // 0x81 DUP2: Duplicate 2nd stack item
        OpcodeHandler::new(HandlerType::Dup, 3, 4, 0), // 0x82 DUP3: Duplicate 3rd stack item
        OpcodeHandler::new(HandlerType::Dup, 4, 5, 0), // 0x83 DUP4: Duplicate 4th stack item
        OpcodeHandler::new(HandlerType::Dup, 5, 6, 0), // 0x84 DUP5: Duplicate 5th stack item
        OpcodeHandler::new(HandlerType::Dup, 6, 7, 0), // 0x85 DUP6: Duplicate 6th stack item
        OpcodeHandler::new(HandlerType::Dup, 7, 8, 0), // 0x86 DUP7: Duplicate 7th stack item
        OpcodeHandler::new(HandlerType::Dup, 8, 9, 0), // 0x87 DUP8: Duplicate 8th stack item
        OpcodeHandler::new(HandlerType::Dup, 9, 10, 0), // 0x88 DUP9: Duplicate 9th stack item
        OpcodeHandler::new(HandlerType::Dup, 10, 11, 0), // 0x89 DUP10: Duplicate 10th stack item
        OpcodeHandler::new(HandlerType::Dup, 11, 12, 0), // 0x8A DUP11: Duplicate 11th stack item
        OpcodeHandler::new(HandlerType::Dup, 12, 13, 0), // 0x8B DUP12: Duplicate 12th stack item
        OpcodeHandler::new(HandlerType::Dup, 13, 14, 0), // 0x8C DUP13: Duplicate 13th stack item
        OpcodeHandler::new(HandlerType::Dup, 14, 15, 0), // 0x8D DUP14: Duplicate 14th stack item
        OpcodeHandler::new(HandlerType::Dup, 15, 16, 0), // 0x8E DUP15: Duplicate 15th stack item
        OpcodeHandler::new(HandlerType::Dup, 16, 17, 0), // 0x8F DUP16: Duplicate 16th stack item
        // 0x90 - 0x9F: Exchange Operations
        // Note: For SWAP operations, in_args is how deep to swap with + 1
        OpcodeHandler::new(HandlerType::Swap, 2, 2, 0), // 0x90 SWAP1: Exchange 1st and 2nd stack items
        OpcodeHandler::new(HandlerType::Swap, 3, 3, 0), // 0x91 SWAP2: Exchange 1st and 3rd stack items
        OpcodeHandler::new(HandlerType::Swap, 4, 4, 0), // 0x92 SWAP3: Exchange 1st and 4th stack items
        OpcodeHandler::new(HandlerType::Swap, 5, 5, 0), // 0x93 SWAP4: Exchange 1st and 5th stack items
        OpcodeHandler::new(HandlerType::Swap, 6, 6, 0), // 0x94 SWAP5: Exchange 1st and 6th stack items
        OpcodeHandler::new(HandlerType::Swap, 7, 7, 0), // 0x95 SWAP6: Exchange 1st and 7th stack items
        OpcodeHandler::new(HandlerType::Swap, 8, 8, 0), // 0x96 SWAP7: Exchange 1st and 8th stack items
        OpcodeHandler::new(HandlerType::Swap, 9, 9, 0), // 0x97 SWAP8: Exchange 1st and 9th stack items
        OpcodeHandler::new(HandlerType::Swap, 10, 10, 0), // 0x98 SWAP9: Exchange 1st and 10th stack items
        OpcodeHandler::new(HandlerType::Swap, 11, 11, 0), // 0x99 SWAP10: Exchange 1st and 11th stack items
        OpcodeHandler::new(HandlerType::Swap, 12, 12, 0), // 0x9A SWAP11: Exchange 1st and 12th stack items
        OpcodeHandler::new(HandlerType::Swap, 13, 13, 0), // 0x9B SWAP12: Exchange 1st and 13th stack items
        OpcodeHandler::new(HandlerType::Swap, 14, 14, 0), // 0x9C SWAP13: Exchange 1st and 14th stack items
        OpcodeHandler::new(HandlerType::Swap, 15, 15, 0), // 0x9D SWAP14: Exchange 1st and 15th stack items
        OpcodeHandler::new(HandlerType::Swap, 16, 16, 0), // 0x9E SWAP15: Exchange 1st and 16th stack items
        OpcodeHandler::new(HandlerType::Swap, 17, 17, 0), // 0x9F SWAP16: Exchange 1st and 17th stack items
        // 0xA0 - 0xA4: Logging Operations
        OpcodeHandler::new(HandlerType::Unimplemented, 2, 0, 0), // 0xA0 LOG0: Append log record with no topics
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 0, 0), // 0xA1 LOG1: Append log record with one topic
        OpcodeHandler::new(HandlerType::Unimplemented, 4, 0, 0), // 0xA2 LOG2: Append log record with two topics
        OpcodeHandler::new(HandlerType::Unimplemented, 5, 0, 0), // 0xA3 LOG3: Append log record with three topics
        OpcodeHandler::new(HandlerType::Unimplemented, 6, 0, 0), // 0xA4 LOG4: Append log record with four topics
        // 0xA5 - 0xEF: Invalid range
        // Fill with unimplemented handlers
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xA5 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xA6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xA7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xA8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xA9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAA (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAC (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAD (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAE (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xAF (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB0 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB1 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB2 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB3 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB4 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB5 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xB9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBA (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBC (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBD (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBE (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xBF (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC0 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC1 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC2 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC3 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC4 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC5 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xC9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCA (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCC (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCD (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCE (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xCF (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD0 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD1 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD2 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD3 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD4 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD5 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xD9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDA (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDC (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDD (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDE (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xDF (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE0 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE1 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE2 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE3 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE4 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE5 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xE9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xEA (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xEB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xEC (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xED (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xEE (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xEF (Invalid)
        // 0xF0 - 0xFF: System operations
        OpcodeHandler::new(HandlerType::Unimplemented, 3, 1, 0), // 0xF0 CREATE: Create new contract
        OpcodeHandler::new(HandlerType::Unimplemented, 7, 1, 0), // 0xF1 CALL: Message-call into account
        OpcodeHandler::new(HandlerType::Unimplemented, 7, 1, 0), // 0xF2 CALLCODE: Message-call with alternative account's code
        OpcodeHandler::new(HandlerType::Terminating, 2, 0, 0), // 0xF3 RETURN: Halt execution returning output data
        OpcodeHandler::new(HandlerType::Unimplemented, 6, 1, 0), // 0xF4 DELEGATECALL: Message-call into this account with an alternative account's code
        OpcodeHandler::new(HandlerType::Unimplemented, 4, 1, 0), // 0xF5 CREATE2: Create new contract with salt
        // 0xF6-0xF9 Invalid
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xF6 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xF7 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xF8 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xF9 (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 6, 1, 0), // 0xFA STATICCALL: Static message-call into account
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xFB (Invalid)
        OpcodeHandler::new(HandlerType::Unimplemented, 0, 0, 0), // 0xFC (Invalid)
        OpcodeHandler::new(HandlerType::Terminating, 2, 0, 0), // 0xFD REVERT: Halt execution reverting state changes
        OpcodeHandler::new(HandlerType::Terminating, 0, 0, 0), // 0xFE INVALID: Invalid instruction
        OpcodeHandler::new(HandlerType::Terminating, 1, 0, 0), // 0xFF SELFDESTRUCT: Halt execution and register account for deletion
    ]
}
