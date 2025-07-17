use std::time::Instant;

use hydra;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use varisat::ExtendFormula;

fn main() {
    (0..16).into_par_iter().for_each(|_| {
        let now = Instant::now();

        for _ in 0..256 {
            run_random();
        }

        let elapsed = now.elapsed();

        println!("Took {:.3} seconds", elapsed.as_millis() as f64 / 1000.0);
    });
}

fn run_random() {
    let mut formula = hydra::Formula::new();

    loop {
        let clause = hydra::Clause::random(3, 0..9).unwrap();
        formula.add_clause(clause.clone());

        let solution = hydra::solve(&formula);
        let sat = solution.is_some();

        if sat != varisat_sat(&formula) {
            panic!("disagreement: hydra {}, varisat: {}", sat, !sat);
        }

        if !sat {
            break;
        }
    }
}

fn varisat_sat(formula: &hydra::Formula) -> bool {
    let mut v_formula = varisat::Solver::new();

    for clause in formula.clauses() {
        let mut v_clause = Vec::new();

        for lit in &clause.literals() {
            v_clause.push(varisat::Lit::from_dimacs(lit.to_dimacs()));
        }

        v_formula.add_clause(&v_clause);
    }

    v_formula.solve().expect("error in varisat")
}
