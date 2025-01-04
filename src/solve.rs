
use crate::sym_stack::Expr;
use std::cmp;

#[derive(Debug)]
pub enum DLResult {
    Sat,
    Unsat
}


pub fn solve(exprs: &[Expr]) -> DLResult {
    let n_exprs = exprs.len();
    if n_exprs == 0 {
        panic!("Empty expression list");
    }

    // Figure out how many nodes we have
    let mut max_var = cmp::max(exprs[0].a.0, exprs[0].b.0);
    for expr in exprs.iter().skip(1) {
        max_var = cmp::max(max_var, cmp::max(expr.a.0, expr.b.0));
    }
    let n_nodes = (max_var + 1) as usize;

    // Allocate and initialize adjacency/weight matrix
    let mut adj: Vec<Vec<i64>> = vec![vec![0; n_nodes]; n_nodes];

    // Fill up adjacency/weight matrix
    for expr in exprs {
        adj[expr.a.0 as usize][expr.b.0 as usize] = expr.k.0;
    }

    // ==== Bellman-Ford negative cycle detection ====
    
    // 1. Single source shortest path
    let mut dist: Vec<i64> = vec![0; n_nodes];

    for _ in 1..n_nodes {
        for expr in exprs {
            let u = expr.a.0 as usize;
            let v = expr.b.0 as usize;
            dist[v] = cmp::min(dist[v], dist[u] + adj[u][v]);
        }
    }

    // 2. Negative cycle detection
    let mut has_negative_cycle = false;
    for expr in exprs {
        let u = expr.a.0 as usize;
        let v = expr.b.0 as usize;
        if dist[v] > dist[u] + adj[u][v] {
            has_negative_cycle = true;
            break;
        }
    }

    if has_negative_cycle {
        DLResult::Unsat
    } else {
        DLResult::Sat
    }
}

