use petgraph::graph::Graph;
use petgraph::dot::Dot;
use pyo3::Python;
use crate::tree::game::Game;

pub fn export_dot(game: Game, py: Python) -> String {
    let mut graph = Graph::new();
    let root = graph.add_node(game.root.borrow(py).player.name.clone());
    for child in game.root.borrow(py).children.clone() {
        let node = graph.add_node(child.borrow(py).player.name.clone());
        graph.extend_with_edges(&[
            (root, node, child.borrow(py).name.clone())
        ]);
    }

    Dot::new(&graph).to_string()
}

#[cfg(test)]
mod tests {
    // importing names from outer scope.
    use super::*;
    #[test]
    fn test_graph() {
        //export_dot();
        assert_eq!(1,1);
    }

}
