use std::collections::HashMap;

use crate::{Assignment, Formula, Lit, Var};

#[derive(Debug, Clone)]
enum LitPolarities {
    TrueOnly,
    FalseOnly,
    Both,
}

#[derive(Debug, Clone)]
enum ClauseState {
    Watching(Lit, Lit),
    Unit(Lit),
    Complete(bool),
}

/// The context required to evaluate a stage of solving.
/// Acts as a clause database, aiming to enable fast quering for unit/pure literals, unassigned variables, and formula states.
#[derive(Debug, Clone)]
struct Context<'a> {
    /// Reference to the formula we are solving
    formula: &'a Formula,
    /// The current assignment we are working with
    assignment: Assignment,
    /// A set of variables in the formula.
    unassigned_variables: HashMap<Var, LitPolarities>,
    /// States of the clauses in the formula, indexed in the same order
    clause_states: Vec<ClauseState>,
    // /// A (arbritrary) clause that is not complete
    // watched_clause: Option<usize>,
}

impl<'a> Context<'a> {
    pub fn new(formula: &Formula) -> Context {
        let mut unassigned_variables = HashMap::new();
        let mut clause_states = Vec::new();

        for clause in formula.clauses() {
            for lit in clause.literals() {
                if let Some(polarities) = unassigned_variables.get(&lit.var()) {
                    if match polarities {
                        LitPolarities::TrueOnly => lit.is_negative(),
                        LitPolarities::FalseOnly => lit.is_positive(),
                        LitPolarities::Both => true,
                    } {
                        unassigned_variables.insert(lit.var(), LitPolarities::Both);
                    }
                } else {
                    unassigned_variables.insert(
                        lit.var(),
                        if lit.polarity() {
                            LitPolarities::TrueOnly
                        } else {
                            LitPolarities::FalseOnly
                        },
                    );
                }
            }

            let state = match clause.literals().as_slice() {
                [] => ClauseState::Complete(false),
                [a] => ClauseState::Unit(*a),
                [a, b, ..] => ClauseState::Watching(*a, *b),
            };

            clause_states.push(state);
        }

        Context {
            formula,
            assignment: Assignment::default(),
            unassigned_variables,
            clause_states,
        }
    }

    /// Assigns a variable.
    ///
    /// Returns true if the assignment makes a clause unsat.
    #[must_use]
    pub fn assign(&mut self, var: &Var, value: bool) -> bool {
        self.assignment.set(*var, value);
        self.unassigned_variables.remove(var);

        for (clause, state) in self.formula.clauses().iter().zip(&mut self.clause_states) {
            match state {
                ClauseState::Watching(a, b) => {
                    // We don't care about literals that aren't watched
                    // This does mean some sat clauses are not immediately identified
                    if a.var() != *var && b.var() != *var {
                        continue;
                    }

                    // If one of the literals is true the clause is true
                    if self.assignment.evaluate(a).unwrap_or(false)
                        || self.assignment.evaluate(b).unwrap_or(false)
                    {
                        *state = ClauseState::Complete(true);
                        continue;
                    }

                    debug_assert!(
                        (a.var() == *var || b.var() == *var),
                        "var is not one of the watched lits"
                    );

                    // Find a new unassigned literal to watch
                    let unassigned_lit = if a.var() == *var { *b } else { *a };
                    let mut new_lit = None;
                    let mut complete = false;

                    for lit in clause.literals() {
                        if let Some(eval) = self.assignment.evaluate(&lit) {
                            if eval {
                                *state = ClauseState::Complete(true);
                                complete = true;
                                break;
                            }
                        } else if lit.var() != unassigned_lit.var() {
                            new_lit = Some(lit);
                        }
                    }

                    if complete {
                        continue;
                    }

                    if let Some(new_lit) = new_lit {
                        debug_assert!(!self.assignment.contains(&new_lit.var()));
                        debug_assert_ne!(new_lit.var(), unassigned_lit.var());
                        *state = ClauseState::Watching(unassigned_lit, new_lit);
                    } else {
                        *state = ClauseState::Unit(unassigned_lit);
                    }
                }

                ClauseState::Unit(lit) => {
                    if lit.var() == *var {
                        if lit.evaluate(value) {
                            *state = ClauseState::Complete(true);
                        } else {
                            *state = ClauseState::Complete(false);
                            return true;
                        }
                    }
                }

                ClauseState::Complete(sat) => {
                    if !*sat {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    /// Shortcut for `assign(lit.var(), lit.polarity())`.
    #[must_use]
    pub fn assign_lit(&mut self, lit: &Lit) -> bool {
        self.assign(&lit.var(), lit.polarity())
    }

    /// Gets a unit literal if one exists.
    pub fn get_unit_lit(&self) -> Option<Lit> {
        self.clause_states
            .iter()
            .filter_map(|state| {
                if let ClauseState::Unit(lit) = state {
                    Some(*lit)
                } else {
                    None
                }
            })
            .next()
    }

    /// Gets a pure literal if one exists.
    pub fn get_pure_lit(&self) -> Option<Lit> {
        self.unassigned_variables
            .iter()
            .filter_map(|(var, polarities)| match polarities {
                LitPolarities::TrueOnly => Some(var.positive()),
                LitPolarities::FalseOnly => Some(var.negative()),
                LitPolarities::Both => None,
            })
            .next()
    }

    /// Tries to get an unassigned variable.
    pub fn get_unassigned_var(&self) -> Option<Var> {
        self.unassigned_variables.keys().next().copied()
    }
}

/// Attempts to find a satisfying set of assignments for this formula. Variables not in the returned solution are unassigned and can take any value.
pub fn solve(formula: &Formula) -> Option<Assignment> {
    if formula.clauses().is_empty() {
        return None;
    }

    attempt_solve(Context::new(formula))
}

/// Continues a DPLL solve using known assignments and an assumed value.
fn attempt_solve(mut ctx: Context) -> Option<Assignment> {
    loop {
        let mut changed = false;

        // Unit Propagation
        if let Some(unit_lit) = ctx.get_unit_lit() {
            if ctx.assign_lit(&unit_lit) {
                return None;
            }

            changed = true;
        }

        // Pure Literal Elimination
        if let Some(pure_lit) = ctx.get_pure_lit() {
            if ctx.assign_lit(&pure_lit) {
                return None;
            }

            changed = true;
        }

        if !changed {
            break;
        }

        let mut all_true = true;
        for state in &ctx.clause_states {
            if matches!(state, ClauseState::Complete(false)) {
                unreachable!("this should have been caught earlier")
            }

            if !matches!(state, ClauseState::Complete(true)) {
                all_true = false;
                break;
            }
        }

        if all_true {
            return Some(ctx.assignment);
        }
    }

    // Assume and recurse
    let branch_var = if let Some(var) = ctx.get_unassigned_var() {
        var
    } else {
        return Some(ctx.assignment);
    };

    for branch in [true, false] {
        let mut ctx = ctx.clone();

        if ctx.assign(&branch_var, branch) {
            continue;
        }

        let mut all_true = true;
        for state in &ctx.clause_states {
            if matches!(state, ClauseState::Complete(false)) {
                unreachable!("this should have been caught earlier")
            }

            if !matches!(state, ClauseState::Complete(true)) {
                all_true = false;
                break;
            }
        }

        if all_true {
            return Some(ctx.assignment);
        }

        if let Some(solution) = attempt_solve(ctx) {
            return Some(solution);
        }
    }

    None
}
