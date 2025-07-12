# HYDA

Hydra is a DPLL SAT solver written in Rust.

It takes a boolean formula in Conjunctive Normal Form (CNF) and attempts to find a satisfying solution to it.
A CNF formula has a number of clauses containing literals, where each clause is a disjunction of literals and the formula is a conjunction of clauses.
We use the standard notation of numbers to represent each literal, with `-x` being the complement of some variable `x`.

## Usage
Create a formula and add clauses:
```rust
let mut f = Formula::new();

f.add_clause([1, 2].into());
f.add_clause([3, 4].into());
f.add_clause([-1, -2].into());
f.add_clause([-3, -4].into());
f.add_clause([-1, -3].into());
f.add_clause([-2, -4].into());
```
Try and find a solution to the formula:
```rust
let solution = f.solve();
```
The `solve()` function returns an option containing `None` if there is no satisfying assignment, or a `Some(HashMap<Var, bool>)` containing a possible satisfying assignment if one exists.

## Features
- [x] Basic DPLL solving
- [ ] Better errors
  - [x] Use `thiserror`
  - [ ] Switch `From`s to `TryFrom`s
- [ ] 2WL for clauses
- [ ] Treat clauses as subsets of larger literal array (as in varisat)
- [ ] CDCL
