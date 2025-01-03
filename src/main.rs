// difference logic
// https://people.eecs.berkeley.edu/~alanmi/publications/other/2013_mihal_smt.pdf

// need to symbolically encode

// concolic execution: combination of concrete testing which is fuzzing, and symbolic execution
// merge the scalability challenges of symbolic execution and randomness of fuzzing

// path constraints collected
// at every JUMPI, ask differential logic solver if condition can ever be true
// DL solver takes expressions in form a - b <= k
// a, b are variables, k is a constant

// represent contraints as weighted graph
// every constraint a - b <= k is an edge a -> b with weight k, check if
// the graph has a negative cycle. bellman ford negative cycle detection

const STACK_SIZE: usize = 1024;

#[derive(Default)]
pub struct EvmSymStack {
    values: Vec<Term>,
    free_top: u16,
}

#[derive(Default)]
pub struct EvmContext {
    code: Vec<u8>,
    sym_stack: EvmSymStack,
    pc: usize,
    path: Vec<u64>,
    constraints: Vec<Expr>,
    counter: u64,
}

// Differential Logic Constraint of the form a - b <= k
#[derive(Default)]
pub struct Expr {
    a: u64,
    b: u64,
    k: i64,
}

enum Kind {
    Concrete,
    Symbolic,
}

pub struct SymVal {
    value: usize,
    kind: Kind,
}

pub struct Term {
    symval: SymVal,
    term: Vec<Term>,
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args().collect();
    let path = args[0].clone();

    // get the bytecode from cli
}

fn run(runtime: Vec<u8>) {
    // Construct a new Symbolic Execution Context
    let context = EvmContext {
        counter: 1,
        ..Default::default()
    };

    // Interpret the runtime bytecode
    while context.pc < context.code.len() {}
}
