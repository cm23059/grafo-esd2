use crate::definicion::RedDeTransporte;

pub fn crear_red_predefinida() -> RedDeTransporte {
    let mut red = RedDeTransporte::new();

    let capital    = red.agregar_nodo("Capital");
    let norte      = red.agregar_nodo("Norte");
    let sur        = red.agregar_nodo("Sur");
    let este       = red.agregar_nodo("Este");
    let oeste      = red.agregar_nodo("Oeste");
    let aeropuerto = red.agregar_nodo("Aeropuerto");

    red.conectar(capital, norte,      15);
    red.conectar(capital, sur,        20);
    red.conectar(capital, oeste,      10);
    red.conectar(norte,   aeropuerto, 25);
    red.conectar(sur,     este,       30);
    red.conectar(oeste,   norte,       8);
    red.conectar(este,    aeropuerto, 12);

    red
}