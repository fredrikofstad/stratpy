use pyo3::{prelude::*};

#[pyclass]
#[derive(Clone)]
pub struct Decision {
    #[pyo3(get)] player: Player, // make nature own struct?
    #[pyo3(get, set)] pub name: String,
    #[pyo3(get)] pub children: Vec<Py<Decision>>,
}

#[pymethods]
impl Decision {
    #[new]
    pub fn new(player: Player, name: String) -> Self {
        Decision{
            player,
            name,
            children: Vec::new(),
        }
    }
    pub fn get_ref(&self, py: Python) -> Py<Decision>{
        Py::new(py, self.clone()).unwrap()
    }
    pub fn add_child(&mut self, decision: Decision, py: Python){
        self.children.push(Py::new(py, decision).unwrap());
    }
    pub fn __add__(&mut self, other: &Decision, py: Python) -> Py<Decision> {
        self.add_child(other.clone(), py);
        self.get_ref(py)
    }
    pub fn __str__(&self) -> String {
        format!("(player: {} action: {})", self.player.name, self.name)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Player {
    #[pyo3(get)] name: String,
}


#[pymethods]
impl Player {
    #[new]
    pub fn new(name: Option<String>) -> Self {
        Player{ name: name.unwrap_or("root".to_string()), }
    }
}