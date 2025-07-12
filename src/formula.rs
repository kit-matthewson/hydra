//! Clauses and Formulas

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{Lit, Var};

type Assignments = HashMap<Var, bool>;

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
    pub fn is_unit(&self, assignments: &Assignments) -> Option<Lit> {
        let unassigned: Vec<&Lit> = self
            .literals
            .iter()
            .filter(|lit| assignments.contains_key(&lit.var()))
            .collect();

        if unassigned.len() == 1 {
            return unassigned.get(0).map(|l| **l);
        }

        return None;
    }

    /// Attempts to evaluate this clause.
    ///
    /// Returns `None` if evaluation is not possible.
    pub fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        let mut decided = true;

        for lit in &self.literals {
            match assignments.get(&lit.var()) {
                Some(&assignment) => {
                    match assignment == lit.polarity() {
                        true => return Some(true),
                        false => continue,
                    }
                }

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
                .fold(String::new(), |acc, lit| format!("{} {:?}", acc, lit))
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
                .fold(String::new(), |acc, lit| format!("{} {}", acc, lit))
        )
    }
}

#[derive(Default, Clone)]
pub struct Formula {
    clauses: Vec<Clause>,
    variables: HashSet<Var>,
}

impl Formula {
    /// Creates a new, empty, formula.
    pub fn new() -> Formula {
        Formula::default()
    }

    /// Adds a clause to this formula.
    pub fn add_clause(&mut self, clause: Clause) {
        for lit in &clause.literals {
            self.variables.insert(lit.var());
        }

        self.clauses.push(clause);
    }

    /// Attempts to evaluate the formula using the given assignments.
    pub fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
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

    /// Attempts to find a satisfying set of assignments for this formula.
    pub fn solve(&self) -> Option<Assignments> {
        // We clone this so solve does not have to mutate this formula
        self.clone().attempt_solve(HashMap::new())
    }

    /// Continues a DPLL solve using known assignments and an assumed value.
    fn attempt_solve(&mut self, mut assignments: Assignments) -> Option<Assignments> {
        // Unit propogation
        while let Some(unit_lit) = self.find_unit(&assignments) {
            assignments.insert(unit_lit.var(), unit_lit.polarity());
            self.propogate_literal(unit_lit);
        }

        // Pure literal elimination
        while let Some(pure_lit) = self.pure_literals().get(0) {
            assignments.insert(pure_lit.var(), pure_lit.polarity());
            self.propogate_literal(*pure_lit);
        }

        // Stopping conditions
        if self.clauses.is_empty() {
            return Some(assignments);
        } else if self.clauses.iter().any(|clause| clause.is_empty()) {
            return None;
        }

        // Assume and recurse
        let var = self.variables.iter().next().unwrap().clone();

        assignments.insert(var, true);

        if let Some(assignment) = self.attempt_solve(assignments.clone()) {
            return Some(assignment);
        }

        assignments.insert(var, false);

        if let Some(assignment) = self.attempt_solve(assignments) {
            return Some(assignment);
        }

        return None;
    }

    /// Tries to find a unit clause and literal based on the given assignments.
    fn find_unit(&self, assignments: &Assignments) -> Option<Lit> {
        for clause in &self.clauses {
            if let Some(unit_lit) = clause.is_unit(assignments) {
                return Some(unit_lit);
            }
        }

        None
    }

    /// Finds all pure literals (literals that appear with exactly one polarity in all clauses).
    fn pure_literals(&self) -> Vec<Lit> {
        let literals: Vec<Lit> = self
            .clauses
            .iter()
            .map(|clause| clause.literals())
            .flatten()
            .collect();

        let mut pure = Vec::new();

        for lit in &literals {
            if !literals.contains(&lit.complement()) && !pure.contains(lit) {
                pure.push(*lit)
            }
        }

        pure
    }

    /// Propogates a literal, removing clauses that contain it and removing it from clauses that contain its complement.
    fn propogate_literal(&mut self, lit: Lit) {
        self.clauses = self
            .clauses
            .drain(..)
            .filter(|clause| !clause.contains_literal(&lit))
            .collect();

        for clause in &mut self.clauses {
            clause.remove_literal(!lit);
        }

        self.variables.remove(&lit.var());
    }
}
