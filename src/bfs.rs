use petgraph::graph::NodeIndex;
use std::collections::{VecDeque, HashMap};
use crate::definicion::RedDeTransporte;

pub fn bfs_ruta_corta(red: &RedDeTransporte, inicio: NodeIndex, destino: NodeIndex) -> Option<Vec<NodeIndex>> {
    let mut visitados = HashMap::new();
    let mut cola = VecDeque::new();
    
    cola.push_back(inicio);
    visitados.insert(inicio, None); 
    
    while let Some(actual) = cola.pop_front() {
        if actual == destino {
            let mut ruta = Vec::new();
            let mut paso = actual;
            ruta.push(paso);
            
            while let Some(Some(padre)) = visitados.get(&paso) {
                paso = *padre;
                ruta.push(paso);
            }
            
            ruta.reverse();
            return Some(ruta);
        }
        
        for vecino in red.grafo.neighbors(actual) {
            if !visitados.contains_key(&vecino) {
                visitados.insert(vecino, Some(actual));
                cola.push_back(vecino);
            }
        }
    }
    
    None
}

pub fn imprimir_ruta(red: &RedDeTransporte, ruta: Vec<NodeIndex>) {
    let nombres: Vec<&str> = ruta.iter()
        .map(|&idx| red.grafo[idx])
        .collect();
    
    println!("Ruta encontrada ({} paradas): {}", nombres.len(), nombres.join(" -> "));
}