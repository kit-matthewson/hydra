//! Hydra is a basic [DPLL][dpll] based SAT solver.
//! It takes formulae in [CNF][cnf] and attempts to find a satisfying assignment.
//! Inspired by [varisat]
//!
//! [dpll]: https://en.wikipedia.org/wiki/DPLL_algorithm
//! [cnf]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
//! [varisat]: https://github.com/jix/varisat

#![warn(missing_docs)]

use std::{collections::HashSet, fmt};

/// A literal with an index and sign.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct Lit {
    // The index. Following convention, is stored 0-indexed but displayed 1-indexed.
    index: u32,
    // The sign of this literal
    // `true` interpreted as positive, `false` as negative
    value: bool,
}

impl Lit {
    /// Creates a new literal from an `i64`. Negative values interpreted as negative.
    /// `v` must be != 0, otherwise returns `Err`.
    pub fn new(v: i64) -> Result<Lit, ()> {
        if v == 0 {
            return Err(());
        }

        Ok(Lit {
            index: v.unsigned_abs() as u32 - 1,
            value: v.is_positive(),
        })
    }

    pub fn from_index_value(index: u32, value: bool) -> Lit {
        Lit { index, value }
    }

    pub fn neg(&self) -> Lit {
        Lit {
            index: self.index,
            value: !self.value,
        }
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value {
            write!(f, "+{}", self.index + 1)
        } else {
            write!(f, "-{}", self.index + 1)
        }
    }
}

impl TryFrom<i64> for Lit {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Lit::new(value)
    }
}

/// A CNF Formula.
#[derive(Clone, Default, Eq, PartialEq)]
struct Formula {
    formula: Vec<Vec<Lit>>,
    variables: HashSet<u32>,
}

impl Formula {
    pub fn new() -> Self {
        Formula::default()
    }

    pub fn add_clause<Item>(&mut self, literals: &[Item])
    where
        Item: Copy,
        Lit: TryFrom<Item>,
    {
        let clause: Vec<Lit> = literals
            .iter()
            .filter_map(|&item| Lit::try_from(item).ok())
            .collect();

        for lit in &clause {
            self.variables.insert(lit.index);
        }

        self.formula.push(clause);
    }

    pub fn update_variable_list(&mut self) {
        for variable in &self.variables.clone() {
            let occurences: Vec<&Lit> = self
                .formula
                .iter()
                .flatten()
                .filter(|l| l.index == *variable)
                .collect();

            if occurences.is_empty() {
                self.variables.remove(variable);
            }
        }
    }

    pub fn unit_clauses(&self) -> Vec<usize> {
        self.formula
            .iter()
            .enumerate()
            .filter_map(|(i, clause)| if clause.len() == 1 { Some(i) } else { None })
            .collect()
    }

    pub fn pure_literals(&self) -> Vec<Lit> {
        let mut pure_literals: Vec<Lit> = Vec::new();

        for variable in &self.variables {
            let occurences: Vec<&Lit> = self
                .formula
                .iter()
                .flatten()
                .filter(|l| l.index == *variable)
                .collect();

            let is_pure = !(occurences.contains(&&Lit::from_index_value(*variable, true))
                && occurences.contains(&&Lit::from_index_value(*variable, false)));

            if is_pure {
                pure_literals.push(**occurences.get(0).expect("variable has no occurrences"));
            }
        }

        pure_literals
    }

    pub fn propogate_literal(&mut self, literal: Lit) {
        // Remove clauses containing the literal
        self.formula = self
            .formula
            .drain(..)
            .filter(|clause| !clause.contains(&literal))
            .collect();

        // Remove the negation of the literal from clauses
        self.formula = self
            .formula
            .drain(..)
            .map(|clause| clause.into_iter().filter(|l| *l != literal.neg()).collect())
            .collect();

        self.update_variable_list();
    }
}

impl<Clauses, Item> From<Clauses> for Formula
where
    Clauses: IntoIterator<Item = Item>,
    Item: std::borrow::Borrow<[Lit]>,
{
    fn from(clauses: Clauses) -> Self {
        let mut f = Self::new();

        for clause in clauses {
            f.add_clause(clause.borrow());
        }

        f
    }
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, clause) in self.formula.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            for (j, lit) in clause.iter().enumerate() {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:?}", lit)?;
            }
        }

        Ok(())
    }
}

struct Assignment {
    index: u32,
    value: bool,
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.index + 1,
            if self.value { "true" } else { "false" }
        )
    }
}

struct Solver {
    formula: Formula,
}

impl Solver {
    pub fn new(formula: Formula) -> Solver {
        Solver { formula }
    }

    pub fn formula(&self) -> Formula {
        self.formula.clone()
    }

    pub fn solve(&mut self) -> Option<Vec<Assignment>> {
        let mut assignments = Vec::new();

        // Unit propogation
        while let Some(unit_index) = self.formula.unit_clauses().get(0) {
            let unit_lit = self
                .formula
                .formula
                .get(*unit_index)
                .unwrap()
                .get(0)
                .unwrap()
                .clone();

            assignments.push(Assignment {
                index: unit_lit.index,
                value: unit_lit.value,
            });

            println!("Propogating Literal: {:?}", unit_lit);
            self.formula.propogate_literal(unit_lit);
        }

        // Pure literal elimination
        while let Some(pure_literal) = self.formula.pure_literals().get(0) {
            assignments.push(Assignment {
                index: pure_literal.index,
                value: pure_literal.value,
            });

            println!("Eliminating Pure Literal: {:?}", pure_literal);
            self.formula.propogate_literal(*pure_literal);
        }

        // Stopping conditions
        if self.formula.formula.is_empty() {
            return Some(assignments)
        }

        if self.formula.formula.contains(&Vec::new()) {
            return None
        }

        todo!("Recursion not implemented")

    }
}

fn main() {
    let mut f = Formula::new();

    f.add_clause(&[1, 2, 3]);
    f.add_clause(&[2, -3, 1]);

    let mut solver = Solver::new(f);

    println!("Before:\n{:?}\n", solver.formula());
    let assignment = solver.solve();
    println!("\nAfter:\n{:?}", solver.formula());

    println!("Satisfying Assignment: {:?}", assignment);
}
