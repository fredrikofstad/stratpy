use pyo3::{prelude::*};
use pyo3::class::basic::CompareOp;
use std::{sync::atomic::{AtomicUsize, Ordering}};

// Structs and methods to be used in python

#[pyclass]
pub struct Game {
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    players: u16,
    #[pyo3(get)]
    gametype: Type,
    #[pyo3(get)]
    root: Option<Decision>,
}

//#[pyo3(get, set)]

#[pymethods]
impl Game {
    #[new]
    fn new(title: Option<String>, players: Option<u16>, gametype: Option<Type>) -> Self {
        Game{ 
            // TODO: Decide defaults
            title: title.unwrap_or("Untitled Game"),
            players: players.unwrap_or(2),
            gametype: gametype.unwrap_or(Type::Normal),
            root: None
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Type {
    Normal,
    Extensive,
}

struct Utility {
    variable: Variable,
    numeral: i32,
}

// atomic usize for ids
static VAR_ID: AtomicUsize = AtomicUsize::new(0);


// Make utility seperate from variable struct?

#[pyclass]
#[derive(Clone)]
pub struct Variable {
    #[pyo3(get)] // don't need getters in release
    name: String,
    #[pyo3(get)]
    id: usize,
    #[pyo3(get)]
    lower: Vec<usize>,
    #[pyo3(get)]
    higher: Vec<usize>,
    #[pyo3(get)]
    equal: Vec<usize>,
}

#[pymethods]
impl Variable {
    #[new]
    pub fn new(name: String) -> Self {
        Variable{
            name, 
            id: VAR_ID.fetch_add(1, Ordering::SeqCst), 
            lower: Vec::new(), 
            higher: Vec::new(), 
            equal: Vec::new()
        }
    }

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

//Because these types are references, in some situations 
//the Rust compiler may ask for lifetime annotations. If this is the case, you should use Py<PyAny>

#[pyclass]
#[derive(Clone)]
pub struct Decision {
    player: Player, // make nature own struct?
    name: String,
    children: Option<Box<Decision>>,
}

#[pymethods]
impl Decision {
    #[new]
    pub fn new(player: Player, name: String) -> Self {
        Decision{
            player, 
            name, 
            children: None,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Player {
    name: String,
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    
    // importing names from outer scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

}