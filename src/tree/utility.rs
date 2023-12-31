use pyo3::{prelude::*};
use pyo3::class::basic::CompareOp;
use std::{sync::atomic::{AtomicUsize, Ordering}};

#[derive(Clone)]
pub enum Utility {
    Variable(Vec<Variable>),
    Numeral(Vec<i32>),
    None,
}

// atomic usize for ids
// consider pushing references to the variables themselves
static VAR_ID: AtomicUsize = AtomicUsize::new(0);

#[pyclass]
#[derive(Clone)]
pub struct Variable {
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub id: usize,
    #[pyo3(get)] pub lower: Vec<usize>,
    #[pyo3(get)] pub higher: Vec<usize>,
    #[pyo3(get)] pub equal: Vec<usize>,
}

#[pymethods]
impl Variable {
    #[new]
    // TODO: variables should also be pyrefs
    // Variable preferences are stored in vec, and are used when comparing
    // preferences during backwards induction.
    pub fn new(name: String) -> Self {
        Variable{
            name,
            id: VAR_ID.fetch_add(1, Ordering::SeqCst),
            lower: Vec::new(),
            higher: Vec::new(),
            equal: Vec::new()
        }
    }
    fn __repr__(&self) -> String {
        self.name.clone()
    }

    // overloads python's comparison operators in order to let the user
    // define preferences. a > b when a is preffered over b, and == when the
    // player is indifferent to the two outcomes.
    fn __richcmp__(&mut self, other: &Self, op: CompareOp, py: Python) -> PyObject {
        match op {
            // TODO: check for duplicates before pushing
            // TODO: add transitive feature if a > b and b > or == c then a > c!
            CompareOp::Lt => self.higher.push(other.id),
            CompareOp::Eq => self.equal.push(other.id),
            CompareOp::Gt => self.lower.push(other.id),
            _ => (),
        }

        other.clone().into_py(py)

    }

}