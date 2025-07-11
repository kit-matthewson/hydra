//! Hydra is a basic [DPLL][dpll] based SAT solver.
//! It takes formulae in [CNF][cnf] and attempts to find a satisfying assignment.
//! Inspired by [varisat]
//!
//! [dpll]: https://en.wikipedia.org/wiki/DPLL_algorithm
//! [cnf]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
//! [varisat]: https://github.com/jix/varisat

#![warn(missing_docs)]

use std::fmt;

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
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value {
            write!(f, " {}", self.index + 1)
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

        self.formula.push(clause);
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

fn main() {
    let mut f = Formula::new();

    f.add_clause(&[-1, 2, 3]);
    f.add_clause(&[2, -3]);

    println!("{:?}", f);
}
