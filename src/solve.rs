



use crate::sym_stack::Expr;

pub enum SolveResult {
    Sat,
    Unsat
}





fn solve(exprs: &[Expr]) -> SolveResult {
    let mut max_var = std::cmp::max(exprs[0].a.0, exprs[0].b.0);
    for i in 1..exprs.len() {
        max_var = std::cmp::max(max_var, std::cmp::max(exprs[i].a.0, exprs[i].b.0));
    }

    let nodes = max_var + 1;



    todo!()
}
