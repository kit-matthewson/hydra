#[allow(unused_imports)]
use hydra;

fn main() {
    let mut f = hydra::Formula::new();

   // Each pigeon must be in at least one hole
    f.add_clause([1, 2].into()); // pigeon 1
    f.add_clause([3, 4].into()); // pigeon 2

    // Each pigeon in at most one hole
    f.add_clause([-1, -2].into()); // pigeon 1
    f.add_clause([-3, -4].into()); // pigeon 2

    // No two pigeons in the same hole
    f.add_clause([-1, -3].into()); // hole 1
    f.add_clause([-2, -4].into()); // hole 2

    let f = f;

    let solution = f.solve();
    println!("{:?}", solution);

    if let Some(solution) = solution {
        println!("{:?}", f.evaluate(&solution));
    }
}
