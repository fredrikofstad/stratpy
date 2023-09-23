use std::fmt;
use std::fmt::format;
use pyo3::{prelude::*};
use pyo3::types::PyString;
use crate::node::*;

#[pyclass]
#[derive(Clone)]
pub struct Game {
    #[pyo3(get)]
    title: String,
    #[pyo3 (get)]
    players: Vec<Player>,
    #[pyo3(get)]
    gametype: Type,
    #[pyo3(get)]
    root: Py<Decision>,
}

//#[pyo3(get, set)]

#[pymethods]
impl Game {
    #[new]
    fn new(title: Option<String>, gametype: Option<Type>, py:Python) -> Self {
        Game{
            // TODO: make function that automakes players
            title: title.unwrap_or("Untitled Game".parse().unwrap()),
            players: Vec::new(),
            gametype: gametype.unwrap_or(Type::Normal),
            root: Py::new(py,Decision::new(Player::new(None), String::from("root"))).unwrap(),
        }
    }
    pub fn get_ref(&self, py: Python) -> Py<Game>{
        Py::new(py, self.clone()).unwrap()
    }
    fn __add__(&mut self, other: &Decision, py: Python) -> Py<Game> {
        self.root.borrow_mut(py).add_child(other.clone(), py);
        self.get_ref(py)
    }
    fn __str__(&self, py: Python) -> String {
        let mut str = format!("title: {} root: {} children: ", self.title, self.root.borrow_mut(py).name);
        let children = self.root.borrow_mut(py).children.clone();
        for child in children{
            str.push_str(&*child.borrow_mut(py).__str__());
        }
        str
    }
    pub fn length(&self, py: Python) -> usize {
        self.root.borrow_mut(py).children.len()
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Type {
    Normal,
    Extensive,
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