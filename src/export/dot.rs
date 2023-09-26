use petgraph::graph::Graph;
use petgraph::dot::Dot;

pub fn export_dot(){
    let mut graph = Graph::new();
    let origin = graph.add_node("Nature");
    let destination_1 = graph.add_node("p");
    let destination_2 = graph.add_node("p-1");

    graph.extend_with_edges(&[
        (origin, destination_1, "die"),
        (origin, destination_2, "live")
    ]);

    let dot_graph = Dot::new(&graph);
    println!("{}",dot_graph.to_string() );
}

#[cfg(test)]
mod tests {
    // importing names from outer scope.
    use super::*;
    #[test]
    fn test_graph() {
        export_dot();
        assert_eq!(1,1);
    }

}
