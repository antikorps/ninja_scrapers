use std::{sync::Arc, time::SystemTime};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{utilidades::Cabeceras, App};

#[derive(Deserialize)]
pub struct Parametros {
    t: i64,
    id: String,
    num: u8,
    offset: u8,
}
#[derive(sqlx::FromRow)]
struct Registro {
    artista: String,
    discografia: String,
    generos: String,
}

#[derive(Serialize)]
pub struct Respuesta {
    #[serde(skip_serializing_if = "Option::is_none")]
    artista: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discografia: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generos: Option<Value>,
}

pub async fn proporcionar_registros(
    Query(parametros): Query<Parametros>,
    cabeceras: HeaderMap,
    State(estado): State<Arc<App>>,
) -> (StatusCode, Json<Vec<Respuesta>>) {
    let mut respuesta_invalida = Vec::new();
    respuesta_invalida.push(Respuesta {
        artista: None,
        discografia: None,
        generos: None,
    });

    let c = Cabeceras(cabeceras);
    // Exigir un agente que empiece por Mozilla
    if !c.agente_valido() {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }
    // Exigir mínimo 6 headers
    if !c.supera_numero_minimo(6) {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }
    // parametros.t tiene que ser 10 segundos arriba/abajo de la actualidad
    let unixtime;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Err(error) => {
            println!("Error en el unixtime {error}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_invalida));
        }
        Ok(ok) => {
            unixtime = ok.as_secs() as i64;
        }
    }

    let diferencia = unixtime - parametros.t;
    if diferencia.abs() > 10 {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }
    // Restricciones numero registros y offset
    if parametros.num > 25 {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }
    if parametros.offset > 105 {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }
    // Identificador discografía completa
    if parametros.id != "discografía completa" {
        return (StatusCode::BAD_REQUEST, Json(respuesta_invalida));
    }

    let consulta = "
        SELECT artista, discografia, generos 
        FROM bbdd
        WHERE id IN (1, 25, 28, 31, 35, 46, 52, 56, 61, 66, 71, 77, 80, 100, 111, 122, 128, 142, 165, 166, 167, 169, 173, 179, 191, 197, 204, 213, 215, 221, 222, 232, 239, 241, 257, 259, 265, 277, 279, 315, 319, 323, 328, 330, 335, 338, 339, 368, 369, 380, 386, 387, 389, 397, 422, 425, 434, 435, 439, 452, 457, 459, 473, 475, 490, 494, 497, 507, 518, 519, 546, 560, 565, 572, 588, 589, 596, 602, 610, 615, 617, 630, 635, 638, 641, 643, 645, 659, 662, 666, 681, 690, 697, 715, 718, 731, 732, 736, 739, 744, 746, 749, 778, 783, 787) 
        ORDER BY id
        LIMIT ?
        OFFSET ?
    ";

    let registros;
    match sqlx::query_as::<_, Registro>(consulta)
        .bind(parametros.num)
        .bind(parametros.offset)
        .fetch_all(&estado.bbdd)
        .await
    {
        Err(error) => {
            println!("Error en la base datos {error}");

            return (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_invalida));
        }
        Ok(ok) => {
            registros = ok;
        }
    }

    let mut respuesta_valida = Vec::new();
    for registro in &registros {
        let serializacion_discografia: Result<Value, serde_json::Error> =
            serde_json::from_str(&registro.discografia);
        let serializacion_generos: Result<Value, serde_json::Error> =
            serde_json::from_str(&registro.generos);
        if serializacion_discografia.is_err() || serializacion_generos.is_err() {
            println!(
                "Error en la serializacion {:#?} {:#?}",
                serializacion_discografia, serializacion_generos
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_invalida));
        }
        respuesta_valida.push(Respuesta {
            artista: Some(String::from(&registro.artista)),
            discografia: Some(serializacion_discografia.unwrap()),
            generos: Some(serializacion_generos.unwrap()),
        })
    }

    return (StatusCode::OK, Json(respuesta_valida));
}
