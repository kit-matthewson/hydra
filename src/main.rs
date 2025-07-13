use core::panic;

#[allow(unused_imports)]
use hydra;

use varisat::{ExtendFormula, Solver};

fn main() {
    for i in 0..100 {
        let mut f = hydra::Formula::new();
        let mut vf = Solver::new();

        loop {
            let clause = hydra::Clause::random(3, 0..9).unwrap();
            f.add_clause(clause.clone());

            let mut vc = Vec::new();
            for lit in clause.literals() {
                vc.push(varisat::Lit::from_dimacs(lit.to_dimacs()));
            }
            vf.add_clause(&vc);

            let sat = f.solve().is_some();
            let vsat = vf.solve().unwrap();

            if sat != vsat {
                println!("{} - {}", sat, vsat);
                panic!("disagreement");
            }

            if !sat {
                println!("run {}: {} clauses", i, f.clauses().len());
                break;
            }
        }
    }
}
