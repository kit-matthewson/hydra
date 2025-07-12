//! Hydra is a basic [DPLL][dpll] based SAT solver.
//! It takes formulae in [CNF][cnf] and attempts to find a satisfying assignment.
//! Inspired by [varisat]
//!
//! [dpll]: https://en.wikipedia.org/wiki/DPLL_algorithm
//! [cnf]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
//! [varisat]: https://github.com/jix/varisat

pub mod errors;
mod formula;
mod literals;

pub use formula::*;
pub use literals::*;
