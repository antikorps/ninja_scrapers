use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};
use handlebars::Handlebars;

use crate::App;

pub async fn servir_index(
    Path(idioma): Path<String>,
    State(app): State<Arc<App>>,
) -> (StatusCode, Html<String>) {
    let plantilla = include_str!("plantilla.html");

    let idioma_esperado = app.idiomas.get(&idioma);
    if idioma_esperado.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Html("<p>Idioma no encontrado. Language not found</p>".to_string()),
        );
    }

    let hb = Handlebars::new();
    match hb.render_template(plantilla, &idioma_esperado.unwrap()) {
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<p>Ha fallado el renderizado de la plantilla: {}. Utiliza, por favor, otro idioma</p>", error)));
        }
        Ok(ok) => {
            return (StatusCode::ACCEPTED, Html(ok));
        }
    }
}
