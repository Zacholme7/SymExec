# SymExec
Almost identical re-write of [this](https://github.com/leonardoalt/dl_symb_exec_sol), but in rust as a learning exercise. It is a simple difference logic symbolic execution engine that encodes constraints into DL expressions that can then be solved using a negative cycle bellman-ford algorithm. 

# Testcase
The `test.bin` is the bytecode [here](https://github.com/leonardoalt/dl_symb_exec_sol/blob/2cff6269a5eb077709f212651fd5da55cbe57d3d/test/SymbExec.t.sol#L67) which also provides a good writeup. It represents the following code 
```yul
{
	let x := calldataload(0)
	if lt(x, 10) {
		if lt(50, x) {
			revert(0, 0)
		}
	}
}
```
When you run the engine with the binary, it will determine that the first branch is satisfiable and the second branch is unsatisfiable. Which makes sense. 
```
cargo run test.bin
```
```
RESULT: Sat
RESULT: Unsat
```



