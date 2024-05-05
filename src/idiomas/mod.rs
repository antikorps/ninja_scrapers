use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Idioma {
    codigo: String,
    textos: Vec<String>,
}

pub fn recopilar_idiomas(puerto: &u16) -> HashMap<String, Idioma> {
    let mut idiomas = HashMap::new();
    idiomas.insert("es", include_str!("es.toml"));
    idiomas.insert("en", include_str!("en.toml"));

    let mut traducciones = HashMap::new();
    for (clave, valor) in idiomas {
        let deserializacion: Result<Idioma, toml::de::Error> = toml::from_str(valor);
        let traduccion;
        match deserializacion {
            Err(error) => {
                eprintln!(
                    "ha fallado la deserialización del idioma {}: {}",
                    clave, error
                );
                continue;
            }
            Ok(ok) => {
                traduccion = ok;
            }
        }

        println!(
            "{} ==> {} http://localhost:{}/{}",
            clave.to_ascii_uppercase(),
            traduccion.textos[0],
            puerto,
            clave
        );
        traducciones.insert(String::from(clave), traduccion);
    }

    return traducciones;
}
