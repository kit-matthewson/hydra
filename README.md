# HYDA

Hydra is a DPLL SAT solver written in Rust.

It takes a boolean formula in Conjunctive Normal Form (CNF) and attempts to find a satisfying solution to it.
A CNF formula has a number of clauses containing literals, where each clause is a disjunction of literals and the formula is a conjunction of clauses.
We use the standard notation of numbers to represent each literal, with `-x` being the complement of some variable `x`.

## Usage
Create a formula:
```rust
let mut f = Formula::new();
```
Add clauses:
```rust
f.add_clause(&[-1, -2]);
f.add_clause(&[-1, -3]);
f.add_clause(&[-2, -3]);
```
Give the formula to a solver and solve:
```rust
let mut solver = Solver::new(f);
let solution = solver.solve();
```
The `solve()` function returns an option containing `None` if there is no satisfying assignment, or `Some(Vec<Assignment>)` containing a possible satisfying assignment if one exists.
