use pyo3::{prelude::*};
use pyo3::types::PyTuple;
use std::{sync::atomic::{AtomicUsize, Ordering}};
use crate::tree::utility::*;

// unique id, given for internal identification when exporting
static VAR_ID: AtomicUsize = AtomicUsize::new(0);

#[pyclass]
#[derive(Clone)]
pub struct Decision {
    #[pyo3(get)] pub player: Py<Player>, // make nature own struct?
    #[pyo3(get, set)] pub name: String,
    #[pyo3(get)] pub children: Vec<Py<Decision>>,
    #[pyo3(get)] pub information_set: Option<usize>,
    #[pyo3(get)] pub label: Option<String>,
    pub utility: Utility,
    pub id: usize,
}

#[pymethods]
impl Decision {
    #[new]
    pub fn new(player: Py<Player>, name: String, utility: Option<Vec<i32>>, variable: Option<Vec<Variable>>,
               information_set: Option<usize>, label: Option<String>, py: Python) -> Py<Decision> {
        Py::new(py, Decision{
            player,
            name,
            children: Vec::new(),
            utility:
            if let Some(value) = utility {
                Utility::Numeral(value)
            } else {
                if let Some(value) = variable {
                    Utility::Variable(value)
                } else {
                    Utility::None
                }
            },
            information_set,
            label,
            id: VAR_ID.fetch_add(1, Ordering::SeqCst),
        }).unwrap()
    }
    pub fn add_node(slf: Py<Decision>, other: Py<Decision>, py: Python) -> Py<Decision>{
        slf.borrow_mut(py).children.push(other.clone());
        slf
    }
    #[pyo3(signature = (*args))]
    pub fn add_nodes(slf: Py<Decision>, py: Python, args: &PyTuple,) -> Py<Decision>{
        for arg in args{
            let decision: Py<Decision> = arg.extract().unwrap();
            slf.borrow_mut(py).children.push(decision);
        }
        slf
    }
    /*
    pub fn add_utility(&mut self, utility: PyObject){
        // TODO: Get values from enum
        self.utility = Option::from(utility);
    }*/

    // overloads + operator making it possible to push new nodes with +.
    // returns a reference to self for continurd pushing with additional +s
    fn __add__(slf: Py<Decision>, other: Py<Decision>, py: Python) -> Py<Decision> {
        slf.borrow_mut(py).children.push(other.clone());
        slf
    }
    pub fn __str__(&self, py: Python) -> String {
        format!("(player: {} action: {})", self.player.borrow_mut(py).name, self.name)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Player {
    #[pyo3(get, set)] pub name: String,
    #[pyo3(get, set)] pub actions: Vec<String>,
}


#[pymethods]
impl Player {
    #[new]
    pub fn new(name: Option<String>, py: Python) -> Py<Player> {
        Py::new(py, Player{ name: name.unwrap_or("player".to_string()),
                actions: Vec::new()}).unwrap()
    }
    fn __repr__(&self) -> String {
        self.name.clone()
    }
}

