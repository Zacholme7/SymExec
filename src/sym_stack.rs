#[derive(Default)]
pub struct EvmSymStack {
    pub values: Vec<Term>,
    pub free_top: usize,
}

#[derive(Clone)]
pub struct Term {
    pub symval: SymVal,
    pub term: Vec<Term>,
}

#[derive(Clone)]
pub struct SymVal {
    pub value: u64,
    pub kind: Kind,
}

#[derive(Clone, PartialEq)]
pub enum Kind {
    Concrete,
    Symbolic,
}

impl EvmSymStack {
    pub fn sym_top(&self) -> Term {
        if self.free_top == 0 {
            panic!("Stack Underflow")
        }

        self.values[self.free_top - 1].clone()
    }

    pub fn sym_push(&mut self, term: Term) {
        if self.free_top >= 1024 {
            panic!("Stack Overflow")
        }
        self.values.push(term);
        self.free_top += 1;
    }

    pub fn sym_pop(&mut self) {
        if self.free_top == 0 {
            panic!("Stack Underflow")
        }
        self.values.pop();
        self.free_top -= 1;
    }

    pub fn sym_dup(&mut self, n: u8) {
        if self.free_top >= 1024 {
            panic!("Stack Overflow");
        }

        if !(1..16).contains(&n) {
            panic!("Invalid Argument");
        }

        if self.free_top < n as usize {
            panic!("Stack Underflow");
        }

        let top = self.sym_top();
        self.values.push(top);
    }

    pub fn sym_swap(&mut self, n: usize) {
        if !(1..16).contains(&n) {
            panic!("Invalid Argument");
        }

        if self.free_top <= n {
            panic!("Stack Underflow");
        }

        let top = self.sym_top();
        self.values[self.free_top - 1] = self.values[self.free_top - 1 - n].clone();
        self.values[self.free_top - 1 - n] = top;
    }
}

// Differential Logic Constraint of the form a - b <= k
#[derive(Default)]
pub struct Expr {
    a: u64,
    b: u64,
    k: i64,
}
