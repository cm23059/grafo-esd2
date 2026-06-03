mod definicion;
fn main() {
let mut red_de_transporte = definicion::RedDeTransporte::new();

    // Recreando exactamente tu ejemplo:
    let a = red_de_transporte.agregar_nodo("A");
    let b = red_de_transporte.agregar_nodo("B");
    let c = red_de_transporte.agregar_nodo("C");
    let d = red_de_transporte.agregar_nodo("D");
    
    red_de_transporte.conectar(a, b, 1); // 0 -> 1
    red_de_transporte.conectar(a, c, 1); // 0 -> 2
    red_de_transporte.conectar(b, d, 1); // 1 -> 3
    red_de_transporte.conectar(c, d, 1); // 2 -> 3

    red_de_transporte.mostrar_grafo();
}