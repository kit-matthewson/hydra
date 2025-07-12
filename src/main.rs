#[allow(unused_imports)]
use hydra;

fn main() {
    let mut f = hydra::Formula::new();

    f.add_clause([1, 2].into());
    f.add_clause([3, 4].into());
    f.add_clause([-1, -2].into());
    f.add_clause([-3, -4].into());
    f.add_clause([-1, -3].into());
    f.add_clause([-2, -4].into());

    let f = f;

    let solution = f.solve();

    if let Some(solution) = solution {
        println!("{:?} ={:?}", solution, f.evaluate(&solution).unwrap_or(false));
    } else {
        println!("unsat")
    }
}
