use std::io::Error;
use pyo3::{prelude::*};
use crate::export::dot;
use crate::export::latex;
use crate::tree::node::*;
use crate::tree::utility::Variable;

#[pyclass]
#[derive(Clone)]
pub struct Game {
    #[pyo3(get)]
    pub title: String,
    #[pyo3 (get, set)]
    pub player: Vec<Py<Player>>,
    #[pyo3(get)]
    pub utility: Option<Vec<Vec<Vec<i32>>>>,
    #[pyo3(get)]
    pub variable: Option<Vec<Vec<Vec<Variable>>>>,
    #[pyo3(get)]
    pub root: Py<Decision>,
}

#[pymethods]
impl Game {
    #[new]
    // currently all fields are optional and will create sensible defaults.
    // TODO: infer gametype based on input.
    // TODO: overload new function with support for matrix input
    //#[pyo3(signature = (title="Untitled Game", normal=None, players=2))]
    fn new(title: Option<String>, utility: Option<Vec<Vec<Vec<i32>>>>, variable: Option<Vec<Vec<Vec<Variable>>>>, players: Option<usize>,  py:Python) -> Self {
        Game{
            title: title.unwrap_or("Untitled Game".to_string()),
            utility,
            variable,
            player: create_players(players.unwrap_or(2), py),
            root: Decision::new(Player::new(None, py), String::from("root"),
                                None, None, None, None, py),
        }
    }
    pub fn export(&self, py: Python) -> String {
        dot::export_dot(self.clone(), py)
    }
    pub fn export_latex(&self, scale: Option<f32>, filename: Option<&str>, py: Python) -> Result<(), Error>{
        let is_normal = match (&self.utility, &self.variable) {
            (Some(_), _) | (_, Some(_)) => true,
            _ => false,
        };
        let scale = scale.unwrap_or(2.5);
        match filename {
            None => {
                latex::to_terminal(self.clone(), scale, is_normal, py);
                Ok(())
            },
            Some(filename) => latex::write_to_file(self.clone(), scale, filename, is_normal, py),
        }.expect("Error");
        Ok(())
    }

    // TODO: consider removing the abstraction
    pub fn get_ref(&self, py: Python) -> Py<Game>{
        Py::new(py, self.clone()).unwrap()
    }
    // overloads + to be used to for adding nodes to game.
    // returns a python reference to game in order to continue adding
    // nodes successively. The reference to root is borrowed temporarily
    // as a mutable in order to push nodes to children.
    fn __add__(&mut self, other: Py<Decision>, py: Python) -> Py<Game> {
        self.root.borrow_mut(py).children.push(other.clone());
        self.get_ref(py)
    }
    // python's toString method
    // currently returns a string representation of game, and root's children
    fn __str__(&self, py: Python) -> String {
        let mut str = format!("title: {} root: {} children: ", self.title, self.root.borrow_mut(py).name);
        let children = self.root.borrow_mut(py).children.clone();
        for child in children{
            str.push_str(&*child.borrow_mut(py).__str__(py));
        }
        str
    }
    // The length of root's children used during testing.
    pub fn length(&self, py: Python) -> usize {
        self.root.borrow_mut(py).children.len()
    }
}

// Creates a vector of players, where the 0th element is reserved for nature
pub fn create_players(player_num: usize, py: Python) -> Vec<Py<Player>>{
    let mut players: Vec<Py<Player>> = Vec::new();
    players.push(Player::new(Option::from("Nature".to_string()), py));
    for i in 0..player_num {players.push(Player::new(format!("Player {}", i + 1).into(), py))}
    players
}

// Consider: This might not have to be exposed to the user
// but can be inferred and methods can be adjusted accordingly
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