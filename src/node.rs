use pyo3::{prelude::*};

#[pyclass]
#[derive(Clone)]
pub struct Decision {
    #[pyo3(get)] player: Player, // make nature own struct?
    #[pyo3(get, set)] pub name: String,
    #[pyo3(get)] pub children: Vec<Py<Decision>>,
    #[pyo3(get)] pub parent: Option<Py<Decision>>,
}

#[pymethods]
impl Decision {
    #[new]
    pub fn new(player: Player, name: String, py: Python) -> Py<Decision> {
        Py::new(py, Decision{
            player,
            name,
            children: Vec::new(),
            parent: None
        }).unwrap()
    }

    pub fn get_ref(&self, py: Python) -> Py<Decision>{
        Py::new(py, self.clone()).unwrap()
    }

    fn __add__(slf: Py<Decision>, other: Py<Decision>, py: Python) -> Py<Decision> {
        slf.borrow_mut(py).children.push(other.clone());
        slf
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