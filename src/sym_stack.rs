// Symbolic or Concrete Value
#[derive(Clone, Debug)]
pub struct Term {
    pub sym_val: SymVal,
    pub args: Vec<Term>,
}

#[derive(Clone, Debug)]
pub struct SymVal {
    pub value: u64,
    pub kind: Kind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Concrete,
    Symbolic,
}

// An actual integer constant
#[derive(Debug, Default, Clone)]
pub struct Constant(pub i64);
// A variable is represented by its id
#[derive(Debug, Default, Clone)]
pub struct Variable(pub u64);

// Differential Logic Constraint of the form a - b <= k
#[derive(Default, Debug, Clone)]
pub struct Expr {
    pub a: Variable,
    pub b: Variable,
    pub k: Constant,
}

#[derive(Default, Debug, Clone)]
pub struct EvmSymStack {
    pub values: Vec<Term>,
    pub free_top: usize,
}

impl EvmSymStack {
    /// Retrieve the value at the top of the stack
    pub fn sym_top(&self) -> Term {
        if self.free_top == 0 {
            panic!("Stack Underflow in sym_top")
        }

        self.values[self.free_top - 1].clone()
    }

    /// Push a new value onto the top of the stack
    pub fn sym_push(&mut self, term: Term) {
        if self.free_top >= 1024 {
            panic!("Stack Overflow in sym_push")
        }
        self.values.push(term);
        self.free_top += 1;
    }

    /// Pop a value off of the top of the stack
    pub fn sym_pop(&mut self) {
        if self.free_top == 0 {
            panic!("Stack Underflow in sym_pop")
        }
        self.values.pop();
        self.free_top -= 1;
    }

    /// Duplicate value n on the stack
    pub fn sym_dup(&mut self, n: u8) {
        if self.free_top >= 1024 {
            panic!("Stack Overflow in sym_dup");
        }

        if !(1..=16).contains(&n) {
            panic!("Invalid Argument in sym_dup");
        }

        if self.free_top < n as usize {
            panic!("Stack Underflow in sym_dup");
        }

        let to_duplicate = self.values[self.free_top - n as usize].clone();
        self.values.push(to_duplicate);
        self.free_top += 1;
    }

    /// Swap value n and the top of the stack
    pub fn sym_swap(&mut self, n: usize) {
        if !(1..=16).contains(&n) {
            panic!("Invalid Argument in sym_swap");
        }

        if self.free_top <= n {
            panic!("Stack Underflow in sym_swap");
        }

        let top = self.sym_top();
        self.values[self.free_top - 1] = self.values[self.free_top - 1 - n].clone();
        self.values[self.free_top - 1 - n] = top;
    }
}
