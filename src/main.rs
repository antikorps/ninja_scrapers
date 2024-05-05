use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use axum_embed::ServeEmbed;
use idiomas::Idioma;
use magic_crypt::{new_magic_crypt, MagicCrypt256};
use rust_embed::RustEmbed;
use sqlx::{Pool, Sqlite};
use std::env;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use tokio::net::TcpListener;

mod api;
mod bbdd;
mod idiomas;
mod index;
mod resolver;
mod utilidades;
mod websockets;

#[derive(RustEmbed, Clone)]
#[folder = "src/html"]
struct ArchivosEstaticos;

pub struct App {
    bbdd: Pool<Sqlite>,
    idiomas: HashMap<String, Idioma>,
    magic_crypt_app: MagicCrypt256,
}

#[tokio::main]
async fn main() {
    let mut puerto: u16 = 3000;
    match env::var("NINJA_SCRAPERS_PORT") {
        Err(_) => (),
        Ok(ok) => match ok.parse::<u16>() {
            Err(_) => {
                eprintln!("ATENCIÓN: la variable de entorno NINJA_SCRAPERS_PORT no es correcta, no es una cifra de tipo u16. La aplicación intentará iniciarse en el puerto 3000");
            }
            Ok(puerto_usuario) => {
                puerto = puerto_usuario;
            }
        },
    }

    let idiomas = idiomas::recopilar_idiomas(&puerto);

    let servir_estaticos = ServeEmbed::<ArchivosEstaticos>::new();

    let bbdd = bbdd::crear_bbdd_memoria().await;

    let magic_crypt_app = new_magic_crypt!("20tHeXIceLTuishIberyPLEvENT##", 256);

    let app_compartida = Arc::new(App {
        bbdd,
        idiomas,
        magic_crypt_app,
    });

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        governor_limiter.retain_recent();
    });

    let aplicacion = Router::new()
        .route(
            "/html/reto05/:pagina",
            get(api::reto05::servir_pagina).post(api::reto05::autentificar),
        )
        .layer(GovernorLayer {
            // We can leak this because it is created once and then
            config: Box::leak(governor_conf),
        })
        .route("/:idioma", get(index::servir_index))
        .route("/", get(|| async { Redirect::permanent("/es") }))
        .nest_service("/html", servir_estaticos)
        .route("/api/reto03", post(api::reto03::proporcionar_registros))
        .route("/resolver", post(resolver::resolver_reto))
        .route("/ws", get(websockets::manejador))
        .with_state(app_compartida);

    let direccion = SocketAddr::from(([0, 0, 0, 0], puerto));

    let manejador = TcpListener::bind(direccion).await.unwrap();
    axum::serve(
        manejador,
        aplicacion.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
