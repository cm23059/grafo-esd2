mod definicion;
mod bfs;
mod red_predefinida;

use std::io::{self, Write};
use petgraph::graph::NodeIndex;
use definicion::RedDeTransporte;
use bfs::{bfs_ruta_corta, imprimir_ruta};
use red_predefinida::crear_red_predefinida;

fn leer_linea(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn leer_numero(prompt: &str) -> Option<i32> {
    leer_linea(prompt).parse::<i32>().ok()
}

fn buscar_nodo(red: &RedDeTransporte, entrada: &str) -> Option<NodeIndex> {
    if let Ok(n) = entrada.trim().parse::<usize>() {
        if n < red.grafo.node_count() {
            return Some(NodeIndex::new(n));
        }
    }
    red.grafo
        .node_indices()
        .find(|&i| red.grafo[i].to_lowercase() == entrada.to_lowercase())
}

fn listar_nodos(red: &RedDeTransporte) {
    println!("\n  Nodos en la red:");
    for idx in red.grafo.node_indices() {
        println!("    [{}] {}", idx.index(), red.grafo[idx]);
    }
}

fn to_static(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

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


fn menu_red(red: &mut RedDeTransporte, nombre_red: &str) {
    loop {
        println!("\n  ══════════════════════════════════════");
        println!("   Red activa: {}", nombre_red);
        println!("  ══════════════════════════════════════");
        println!("   1. Ver grafo (formato DOT)");
        println!("   2. Listar nodos");
        println!("   3. Buscar ruta más corta");
        println!("   4. Volver al menú principal");
        println!("  ──────────────────────────────────────");

        match leer_linea("  Opción: ").as_str() {
            "1" => red.mostrar_grafo(),
            "2" => listar_nodos(red),
            "3" => menu_buscar_ruta(red),
            "4" => break,
            _   => println!("  ! Opción no válida."),
        }
    }
}


fn main() {
    println!("\n  ╔══════════════════════════════════════╗");
    println!("  ║     Red de Transporte — BFS          ║");
    println!("  ╚══════════════════════════════════════╝");

    loop {
        println!("\n  ── Menú principal ─────────────────────");
        println!("   1. Usar red predefinida");
        println!("   2. Crear red personalizada");
        println!("   3. Salir");
        println!("  ───────────────────────────────────────");

        match leer_linea("  Opción: ").as_str() {
            "1" => {
                let mut red = crear_red_predefinida();
                menu_red(&mut red, "Predefinida");
            }
            "2" => {
                let mut red = construir_red_personalizada();
                menu_red(&mut red, "Personalizada");
            }
            "3" => {
                println!("\n  Hasta luego.\n");
                break;
            }
            _ => println!("  ! Opción no válida."),
        }
    }
}