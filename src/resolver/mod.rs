use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct SolicitudResolucion {
    identificador: u16,
    a: u32,
    b: u32,
    c: u32,
}
#[derive(Serialize)]
pub struct Comprobacion {
    a: String,
    b: String,
    c: String,
}

fn comprobar_numeros(s: &u32, c: &u32) -> String {
    if s == c {
        return String::from("=");
    }
    if s > c {
        return String::from(">");
    }
    return String::from("<");
}

fn comprobar_respuesta(solicitud: SolicitudResolucion, correcta: (u32, u32, u32)) -> Comprobacion {
    Comprobacion {
        a: comprobar_numeros(&solicitud.a, &correcta.0),
        b: comprobar_numeros(&solicitud.b, &correcta.1),
        c: comprobar_numeros(&solicitud.c, &correcta.2),
    }
}

pub async fn resolver_reto(
    Json(solicitud): Json<SolicitudResolucion>,
) -> (StatusCode, Json<Comprobacion>) {
    match solicitud.identificador {
        1 => {
            /*
               a: número de artistas que empiezan por la letra C: 8
               b: número de grabaciones editadas en 1995: 28
               c: número de veces que aparece el género electronic: 4
            */
            let correcta: (u32, u32, u32) = (8, 28, 4);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        2 => {
            /*
               a: artistas formados entre 1980 y 1985 (ambos inclusive): 11
               b: longitud de caracteres del título más corto: 1
               c: suma más baja de cifras identificador: 137
            */
            let correcta: (u32, u32, u32) = (11, 1, 137);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        3 => {
            /*
               a: número total de registros: 105
               b: numero total de grabaciones: 1867
               c: numero total de generos: 356
            */
            let correcta: (u32, u32, u32) = (105, 1867, 356);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        4 => {
            /*
               a: número total de artistas recomendados: 21
               b: numero de artistas que empiezan por la N: 3
               c: suma del total de letras en los identificadores: 240
            */
            let correcta: (u32, u32, u32) = (21, 3, 240);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        5 => {
            /*
               a: número total de generos: 133
               b: suma de los dos años de formación con más artistas: 1980 + 1989 = 3969
               c: primer año de formación: 1653
            */
            let correcta: (u32, u32, u32) = (133, 3969, 1653);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        6 => {
            /*
               a: número total de artistas que empiezan por R: 11
               b: grabaciones publicadas en 1995: 45
               c: total suma caracteres numéricos: 15157
            */
            let correcta: (u32, u32, u32) = (11, 45, 15157);
            let respuesta = comprobar_respuesta(solicitud, correcta);
            return (StatusCode::OK, Json(respuesta));
        }

        _ => {
            let respuesta = Comprobacion {
                a: String::from("!"),
                b: String::from("!"),
                c: String::from("!"),
            };
            return (StatusCode::BAD_REQUEST, Json(respuesta));
        }
    }
}
