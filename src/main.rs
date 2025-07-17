use hydra;
use varisat::ExtendFormula;

// fn main() {
//     let mut formula = hydra::Formula::new();

//     formula.add_clause([2, 4, -1].into());
//     formula.add_clause([-2, 1, -3].into());

//     let solution = hydra::solve(&formula);
//     println!("{:?}\n\n{:?}", formula.clauses(), solution);
// }
fn main() {
    // TODO verify that assignments are the same / valid
    let mut i = 0;

    loop {
        let mut f = hydra::Formula::new();
        let mut vf = varisat::Solver::new();

        // Add random clauses until unsat
        loop {
            let clause = hydra::Clause::random(5, 0..9).unwrap();
            f.add_clause(clause.clone());

            let mut vc = Vec::new();
            for lit in clause.literals() {
                vc.push(varisat::Lit::from_dimacs(lit.to_dimacs()));
            }
            vf.add_clause(&vc);

            let solution = hydra::solve(&f);
            let sat = solution.is_some();

            let _ = vf.solve();
            let vsolution = vf.model();
            let vsat = vsolution.is_some();

            if sat && vsat {
                println!("{} = {:?} or {:?}", clause, &solution.clone().unwrap().vec(), vsolution);
            }

            if sat != vsat {
                panic!("disagreement: hydra {}, varisat {}", sat, vsat);
            }

            if !sat {
                println!("run {}: {} clauses", i, f.clauses().len());
                break;
            }
        }

        i += 1;
    }
}
