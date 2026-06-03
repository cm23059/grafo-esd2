use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};
//definicion de la estrutura, str para string(el nodo), y i32 para int(numero que guarda cara arista del grafo)
pub struct RedDeTransporte{
    pub grafo: DiGraph<&'static str, i32>,
}

//para inicializar un grafo vacio
impl RedDeTransporte{
    pub fn new() -> Self{
        RedDeTransporte { 
            grafo: DiGraph::<&'static str, i32>::new()
         }
    }
}

impl RedDeTransporte{
    //para agregar un nuevo nodo al grafo
    pub fn agregar_nodo(&mut self, nombre_del_nodo: &'static str) -> NodeIndex {
        self.grafo.add_node(nombre_del_nodo)
    }

    //para conectar los nodos
    pub fn conectar(&mut self, nodo_origen: NodeIndex, nodo_destino: NodeIndex, peso: i32) {
        self.grafo.add_edge(nodo_origen, nodo_destino, peso);
    }

    //para ver el grafo
    pub fn mostrar_grafo(&self){
        println!("Grafo:");
        println!("{:?}", Dot::with_config(&self.grafo, &[Config::EdgeNoLabel]));
    }
}