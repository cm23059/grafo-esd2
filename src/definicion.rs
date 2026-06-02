use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
pub fn definicion_grafo() -> Graph<&'static str, i32> {
    let mut grafo = Graph::<&str, i32>::new();   
    let a = grafo.add_node("A");
    let b = grafo.add_node("B");
    let c = grafo.add_node("C");
    let d = grafo.add_node("D");  
    grafo.add_edge(a, b, 5);
    grafo.add_edge(a, c, 2);
    grafo.add_edge(b, d, 1);
    grafo.add_edge(c, d, 7);
    println!("{:?}", Dot::with_config(&grafo, &[Config::EdgeNoLabel]));
    
    grafo
}
