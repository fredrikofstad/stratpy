use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::Dot;
use pyo3::{Py, Python};
use crate::tree::game::Game;
use crate::tree::node::Decision;

pub fn export_dot(game: Game, py: Python) -> String {
    let mut graph = Graph::new();
    add_nodes_to_graph(game.root.clone(), &mut graph, py);
    Dot::new(&graph).to_string()
}

fn add_nodes_to_graph(decision: Py<Decision>, graph: &mut Graph<String, String>, py: Python) -> (NodeIndex, String) {
    // add nodes and edges
    let index = if decision.borrow(py).clone().children.is_empty() {
        graph.add_node("(0,0)".to_string())
    } else {
        graph.add_node(decision.borrow(py).player.name.clone())
    };
    for child in decision.borrow(py).clone().children {
        let (node_index, name) = add_nodes_to_graph(child, graph, py);
        graph.extend_with_edges(&[
            (index, node_index, name)
        ]);
    }
    (index, decision.borrow(py).name.clone())
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
