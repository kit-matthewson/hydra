//! Variables and Literals
//!
//! A literal is a variable or its complement.

use std::{fmt, ops};

use crate::errors::LitError;

pub type LitIndex = u32;

/// A boolean variable.
///
/// Internally, variables are represented with a 0-based index, but are displayed as a 1-based index to provide parity with literals.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    index: LitIndex,
}

#[allow(dead_code)]
impl Var {
    /// Creates a variable from a 1-indexed number, as used by DIMACS CNF.
    ///
    /// Returns `Err` if `number` is less than 1 or produces a `Var` with an index greater than that of `Var::max_var()`.
    pub fn from_dimacs(number: usize) -> Result<Var, LitError> {
        if number == 0 {
            return Err(LitError::InvalidDimacs);
        }

        Var::from_index(number - 1)
    }

    /// Creates a variable from a 0-based index.
    ///
    /// Returns `None` if `index` produces `Var` with an index greater than that of `Var::max_var()`.
    pub fn from_index(index: usize) -> Result<Var, LitError> {
        if index > Var::max_var().index() {
            return Err(LitError::IndexTooLarge);
        }

        Ok(Var {
            index: index as LitIndex,
        })
    }

    /// The 1-based DIMACS encoding of this variable.
    pub fn to_dimacs(&self) -> isize {
        (self.index + 1) as isize
    }

    /// The 0-based index representing this variable.
    pub const fn index(&self) -> usize {
        self.index as usize
    }

    /// The variable with the largest supported index.
    ///
    /// This allows `Lit` to store polarity information within the index.
    pub const fn max_var() -> Var {
        Var {
            index: LitIndex::max_value() >> 2,
        }
    }

    /// The number of different variables that can be represented.
    ///
    /// Equal to `Var::max_var().index() + 1`.
    pub const fn max_count() -> usize {
        Var::max_var().index() + 1
    }

    /// Creates a positive literal from this variable.
    pub fn positive(&self) -> Lit {
        Lit::from_var(self, true)
    }

    /// Creates a negative literal from this variable.
    pub fn negative(&self) -> Lit {
        Lit::from_var(self, false)
    }
}

/// Create a variable from a DIMACS number. Panics if the number is invalid.
impl From<usize> for Var {
    fn from(number: usize) -> Var {
        Var::from_dimacs(number).expect("invalid DIMACS number for variable")
    }
}

/// Gives DIMACS encoding
impl fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dimacs())
    }
}

/// Uses Debug output
impl fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A boolean literal.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lit {
    /// The code of this literal.
    ///
    /// One more than the index of the underlying variable when positive, or double the index when negative.
    code: LitIndex,
}

#[allow(dead_code)]
impl Lit {
    /// Creates a literal from a variable index and polarity.
    ///
    /// `index` must be less than `Var::max_var().index()`.
    pub fn from_index(index: usize, polarity: bool) -> Result<Lit, LitError> {
        if index > Var::max_var().index() {
            return Err(LitError::IndexTooLarge);
        }

        Ok(Lit {
            code: ((index as LitIndex) << 1) | (polarity as LitIndex),
        })
    }

    /// Creates a literal with given polarity from the provided variable.
    pub fn from_var(var: &Var, polarity: bool) -> Lit {
        Lit::from_index(var.index(), polarity).expect("from_var given a var with invalid index.")
    }

    /// Creates a literal from a DIMACS number. The sign of the number is used as the sign of the literal.
    ///
    /// `number.abs()` must be a valid variable index (non-zero and less than `Var::max_var().index()`)
    pub fn from_dimacs(number: isize) -> Result<Lit, LitError> {
        if number == 0 {
            return Err(LitError::InvalidDimacs);
        }

        Lit::from_index(number.unsigned_abs() - 1, number.is_positive())
    }

    /// The DIMACS encoding of this literal. 1-based, positive for positive literals and negative for negative literals.
    pub fn to_dimacs(&self) -> isize {
        self.var().to_dimacs() * if self.is_positive() { 1 } else { -1 }
    }

    /// The 0-based index of the underlying variable.
    pub fn index(&self) -> usize {
        (self.code >> 1) as usize
    }

    /// The underlying variable of this literal.
    pub fn var(&self) -> Var {
        Var {
            index: self.index() as LitIndex,
        }
    }

    /// The polarity (positive or negative) of this literal.
    pub fn polarity(&self) -> bool {
        (self.code & 1) == 1
    }

    /// Whether this literal is positive.
    pub fn is_positive(&self) -> bool {
        self.polarity()
    }

    /// Whether this literal is negative.
    pub fn is_negative(&self) -> bool {
        !self.polarity()
    }

    /// The complement of this literal.
    pub fn complement(&self) -> Lit {
        !*self
    }
}

impl ops::Not for Lit {
    type Output = Lit;

    fn not(self) -> Self::Output {
        Lit::from_index(self.index(), !self.polarity()).unwrap()
    }
}

/// Create a positive literal from a variable
impl From<Var> for Lit {
    fn from(var: Var) -> Lit {
        var.positive()
    }
}

/// Create a literal from a DIMACS number. Panics if the number is invalid.
impl From<isize> for Lit {
    fn from(number: isize) -> Lit {
        Lit::from_dimacs(number).expect("invalid DIMACS number for literal")
    }
}

/// Gives DIMACS encoding
impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dimacs())
    }
}

/// Uses Debug output
impl fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_positive() {
            write!(f, " {:?}", self)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
