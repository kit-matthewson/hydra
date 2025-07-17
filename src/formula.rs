//! Clauses and Formulas

use core::panic;
use std::{collections::HashMap, fmt, ops::Range};

use rand::prelude::*;

use crate::{errors::LitError, Lit, Var};

#[derive(Debug, Default, Clone)]
pub struct Assignment {
    assignemnts: HashMap<Var, bool>,
}

impl Assignment {
    /// Creates a new, empty, assignment.
    pub fn new() -> Assignment {
        Assignment::default()
    }

    /// Returns `true` if the provided variable exists in this assignment.
    pub fn contains(&self, var: &Var) -> bool {
        self.assignemnts.contains_key(var)
    }

    /// Gets the value assigned to `var` if it has an assignment, otherwise returns `None`.
    pub fn get(&self, var: &Var) -> Option<bool> {
        self.assignemnts.get(var).copied()
    }

    /// Sets the value of `var` to `value` in this assignment.
    ///
    /// Returns `true` if the variable was already set.
    pub fn set(&mut self, var: Var, value: bool) -> bool {
        self.assignemnts.insert(var, value).is_some()
    }

    /// Assigns the unerlying variable of `lit` to `lit.polarity()`.
    ///
    /// Returns `true` if the variable was already set.
    pub fn set_lit(&mut self, lit: &Lit) -> bool {
        self.set(lit.var(), lit.polarity())
    }

    /// Evaluate a literal based on this assignement, if one exists for the underlying variable.
    pub fn evaluate(&self, lit: &Lit) -> Option<bool> {
        if let Some(assignment) = self.get(&lit.var()) {
            return Some(assignment == lit.polarity());
        }

        return None;
    }

    /// Returns this assignment as a vector of assignment pairs, sorted by variable index.
    pub fn vec(&self) -> Vec<(Var, bool)> {
        let mut vec = Vec::new();

        for (var, value) in self.assignemnts.iter() {
            vec.push((*var, *value));
        }

        vec.sort_by_key(|(var, _)| var.index());

        vec
    }

    /// Returns this assignement as a vector of literals.
    pub fn lits(&self) -> Vec<Lit> {
        self.vec().iter().map(|(var, value)| Lit::from_var(var, *value)).collect()
    }

    /// Get a hashmap of variable assignments.
    pub fn hashmap(&self) -> HashMap<Var, bool> {
        self.assignemnts.clone()
    }
}

/// A CNF clause. That is, a disjunction of literals that themselves can be the complement of a variable.
#[derive(Clone, Default)]
pub struct Clause {
    literals: Vec<Lit>,
}

#[allow(dead_code)]
impl Clause {
    /// Creates a new clause containing no literals.
    pub fn new() -> Clause {
        Clause::default()
    }

    /// Generates a random clause of `n` literals.
    pub fn random(n: usize, index_range: Range<usize>) -> Result<Clause, LitError> {
        if n > index_range.clone().len() {
            panic!("n > range");
        }

        let mut rng = rand::rng();
        let mut clause = Clause::new();

        for _ in 0..n {
            loop {
                let lit =
                    Lit::from_index(rng.random_range(index_range.clone()), rng.random_bool(0.5))?;

                if !clause.contains_literal(&lit) && !clause.contains_literal(&lit.complement()) {
                    clause.add_literal(lit);
                    break;
                }
            }
        }

        Ok(clause)
    }

    /// Returns a cloned copy of the literals in this clause.
    pub fn literals(&self) -> Vec<Lit> {
        self.literals.clone()
    }

    /// Adds a literal to this clause.
    pub fn add_literal(&mut self, lit: Lit) {
        self.literals.push(lit);
    }

    /// Removes a literal from this clause.
    pub fn remove_literal(&mut self, lit: Lit) {
        self.literals = self.literals.drain(..).filter(|l| *l != lit).collect()
    }

    /// Checks if this clause contains the given literal.
    pub fn contains_literal(&self, lit: &Lit) -> bool {
        self.literals.contains(lit)
    }

    /// Removes all occurences of a variable from this clause.
    pub fn remove_variable(&mut self, var: Var) {
        self.literals = self
            .literals
            .drain(..)
            .filter(|lit| lit.var() != var)
            .collect();
    }

    /// Checks if this clause is empty (contains no literals).
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    /// Checks if this clause is a unit clause (contains exactly one unassigned literal), based on the provided assignments.
    ///
    /// If it is a unit clause, returns the unit literal, otherwise returns `None`.
    pub fn is_unit(&self, assignment: &Assignment) -> Option<Lit> {
        let unassigned: Vec<&Lit> = self
            .literals
            .iter()
            .filter(|lit| assignment.contains(&lit.var()))
            .collect();

        if unassigned.len() == 1 {
            return unassigned.get(0).map(|l| **l);
        }

        return None;
    }

    /// Attempts to evaluate this clause.
    ///
    /// Returns `None` if evaluation is not possible.
    pub fn evaluate(&self, assignments: &Assignment) -> Option<bool> {
        let mut decided = true;

        for lit in &self.literals {
            match assignments.evaluate(&lit) {
                Some(value) => match value {
                    true => return Some(true),
                    false => continue,
                },

                None => decided = false,
            }
        }

        if decided {
            Some(false)
        } else {
            None
        }
    }
}

impl<Literals, Item> From<Literals> for Clause
where
    Literals: IntoIterator<Item = Item>,
    Item: Into<Lit>,
{
    fn from(literals: Literals) -> Self {
        let mut clause = Clause::new();

        literals
            .into_iter()
            .for_each(|lit| clause.add_literal(lit.into()));

        clause
    }
}

impl fmt::Debug for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.literals
                .iter()
                .rfold(String::new(), |acc, lit| format!("{:?} {}", lit, acc))
        )
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.literals
                .iter()
                .rfold(String::new(), |acc, lit| format!("{:?} {}", lit, acc))
        )
    }
}

#[derive(Default, Clone)]
pub struct Formula {
    clauses: Vec<Clause>,
}

impl Formula {
    /// Creates a new, empty, formula.
    pub fn new() -> Formula {
        Formula::default()
    }

    /// Adds a clause to this formula.
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    /// Gets the clauses in this formula.
    pub fn clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }

    /// Attempts to evaluate the formula using the given assignments.
    pub fn evaluate(&self, assignments: &Assignment) -> Option<bool> {
        let mut decided = true;

        for clause in &self.clauses {
            match clause.evaluate(assignments) {
                Some(true) => continue,
                Some(false) => return Some(false),
                None => decided = false,
            }
        }

        if decided {
            Some(true)
        } else {
            None
        }
    }
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for clause in self.clauses() {
            writeln!(f, "{:?}", clause)?
        }

        Ok(())
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for clause in self.clauses() {
            writeln!(f, "{}", clause)?
        }

        Ok(())
    }
}
