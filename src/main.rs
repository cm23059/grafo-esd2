mod definicion;
mod bfs;
mod red_predefinida;

use std::io::{self, Write};
use petgraph::graph::NodeIndex;
use definicion::RedDeTransporte;
use bfs::{bfs_ruta_corta, imprimir_ruta};
use red_predefinida::crear_red_predefinida;


fn construir_red_personalizada() -> RedDeTransporte {
    let mut red = RedDeTransporte::new();

    println!("\n  ┌──────────────────────────────────────┐");
    println!("  │   Construcción de red personalizada  │");
    println!("  └──────────────────────────────────────┘");

    println!("  Ingresa los nodos de la red.");
    println!("  (Deja el nombre vacío y presiona ENTER para terminar)");

    loop {
        let nombre = leer_linea("\n  Nombre del nodo: ");
        if nombre.is_empty() {
            if red.grafo.node_count() == 0 {
                println!("  ! Debes agregar al menos un nodo.");
                continue;
            }
            break;
        }
        if buscar_nodo(&red, &nombre).is_some() {
            println!("  ! Ya existe un nodo con ese nombre.");
            continue;
        }
        let nombre_estatico = to_static(nombre.clone());
        red.agregar_nodo(nombre_estatico);
        println!("  ✔ Nodo \"{}\" agregado.", nombre);
    }

    listar_nodos(&red);

    println!("\n  Ingresa las conexiones entre nodos.");
    println!("  (Deja el origen vacío y presiona ENTER para terminar)");

    loop {
        let origen_nombre = leer_linea("\n  Nodo origen: ");
        if origen_nombre.is_empty() {
            break;
        }

        let origen = match buscar_nodo(&red, &origen_nombre) {
            Some(idx) => idx,
            None => {
                println!("  ! Nodo \"{}\" no encontrado.", origen_nombre);
                continue;
            }
        };

        let destino_nombre = leer_linea("  Nodo destino: ");
        let destino = match buscar_nodo(&red, &destino_nombre) {
            Some(idx) => idx,
            None => {
                println!("  ! Nodo \"{}\" no encontrado.", destino_nombre);
                continue;
            }
        };

        if origen == destino {
            println!("  ! El origen y el destino no pueden ser el mismo nodo.");
            continue;
        }

        let peso = leer_numero("  Peso / distancia (entero): ").unwrap_or(1);

        red.conectar(origen, destino, peso);
        println!(
            "  Conexión {} → {} (peso: {}) agregada.",
            origen_nombre, destino_nombre, peso
        );
    }

    red
}


fn menu_buscar_ruta(red: &RedDeTransporte) {
    if red.grafo.node_count() == 0 {
        println!("\n  ! La red no tiene nodos.");
        return;
    }

    listar_nodos(red);

    let origen_nombre  = leer_linea("\n  Nodo de origen:  ");
    let destino_nombre = leer_linea("  Nodo de destino: ");

    let origen = match buscar_nodo(red, &origen_nombre) {
        Some(idx) => idx,
        None => {
            println!("  ! Nodo \"{}\" no encontrado.", origen_nombre);
            return;
        }
    };

    let destino = match buscar_nodo(red, &destino_nombre) {
        Some(idx) => idx,
        None => {
            println!("  ! Nodo \"{}\" no encontrado.", destino_nombre);
            return;
        }
    };

    if origen == destino {
        println!("\n  ! El origen y el destino son el mismo nodo.");
        return;
    }

    match bfs_ruta_corta(red, origen, destino) {
        Some(ruta) => imprimir_ruta(red, ruta),
        None => println!(
            "\n  ✘ No existe ruta de \"{}\" a \"{}\" en esta red.",
            origen_nombre, destino_nombre
        ),
    }
}
