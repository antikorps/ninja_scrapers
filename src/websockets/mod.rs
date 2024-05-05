use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use bson;
use serde::Deserialize;
use serde::Serialize;

use crate::App;

#[derive(Serialize, Deserialize)]
struct MensajeWS {
    identificador: String,
    espera: u32,
    fin: bool,
}
#[derive(sqlx::FromRow)]
struct Registro {
    identificador: String,
    artista: String,
}

pub async fn manejador(ws: WebSocketUpgrade, State(estado): State<Arc<App>>) -> Response {
    ws.on_upgrade(|socket| manejador_socket(socket, estado))
}

async fn manejador_socket(mut socket: WebSocket, estado: Arc<App>) {
    let consulta = r###"
    SELECT identificador, artista FROM bbdd
    WHERE id IN (35, 81, 101, 149, 153, 155, 183, 191, 227, 248, 381, 384, 410, 465, 502, 670, 701, 708, 721, 735, 799)
    "###;

    let registros;
    match sqlx::query_as::<_, Registro>(consulta)
        .fetch_all(&estado.bbdd)
        .await
    {
        Err(error) => {
            println!("Error recuperando los registros de la base datos para el reto 04 {error}");
            let registros_vacio: Vec<Registro> = Vec::new();
            registros = registros_vacio
        }
        Ok(ok) => {
            registros = ok;
        }
    }

    match socket.recv().await {
        None => (),
        Some(ok) => {
            match ok {
                Err(_) => {
                    // Cliente desconectado
                    return;
                }
                Ok(mensaje) => {
                    let incorrecto = bson::bson!({
                        "error": true,
                    });
                    let data_incorrecto = bson::to_vec(&incorrecto);
                    if data_incorrecto.is_err() {
                        return;
                    }
                    let mensaje_incorrecto = Message::from(data_incorrecto.unwrap());

                    let data = mensaje.into_data();

                    let mensaje: MensajeWS;
                    match bson::from_slice(&data) {
                        Err(_) => {
                            // Mal serializado
                            let _ = socket.send(mensaje_incorrecto).await;
                            return;
                        }
                        Ok(ok) => {
                            mensaje = ok;
                        }
                    }

                    if mensaje.identificador != "artistas" {
                        let _ = socket.send(mensaje_incorrecto).await;
                        return;
                    }
                    if mensaje.espera < 3 {
                        let _ = socket.send(mensaje_incorrecto).await;
                        return;
                    }

                    for (indice, registro) in registros.iter().enumerate() {
                        let identificador = String::from(&registro.identificador);
                        let artista = String::from(&registro.artista);
                        let mut fin = false;
                        if indice == registros.len() - 1 {
                            fin = true
                        }
                        let respuesta = bson::bson!({
                            "error": false,
                            "identificador": identificador,
                            "artista": artista,
                            "fin": fin
                        });
                        let respuesta_codificada = bson::to_vec(&respuesta);
                        if respuesta_codificada.is_err() {
                            let incorrecto = bson::bson!({
                                "error": true,
                                "mensaje": "bson error",
                            });
                            let mensaje_incorrecto = bson::to_vec(&incorrecto);
                            if mensaje_incorrecto.is_err() {
                                return;
                            }
                            let mensaje = Message::from(mensaje_incorrecto.unwrap());
                            let _ = socket.send(mensaje).await;
                            return;
                        }
                        let respuesta_ws = Message::from(respuesta_codificada.unwrap());
                        let _ = socket.send(respuesta_ws).await;
                        tokio::time::sleep(Duration::from_secs(mensaje.espera.into())).await;
                    }
                }
            }
        }
    }
}
