use pyo3::prelude::*;

// Structs and methods to be used in python

#[pyclass]
pub struct Game {
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    players: u8,
    #[pyo3(get)]
    gametype: GameType,
}

//#[pyo3(get, set)]

impl Default for Game {
    fn default() -> Self {
        Game { title: "Game".to_string(), players: 2, gametype: GameType::Normal}
    }
}

#[pymethods]
impl Game {
    #[new]
    fn new(title: String, players: u8, gametype: GameType) -> Self {
        Game{ title, players, gametype }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum GameType {
    Normal,
    Extensive,
}

struct Utility {
    variable: Variable,
    numeral: i32,
}

struct Variable {
    name: String,
    value: u16,
}

struct Decision {
    player: Player,
    name: String,
    utility: Utility, 
    children: Box<Decision>,
}

struct Player {
    name: String,
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// fail
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}