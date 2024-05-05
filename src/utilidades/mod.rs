use std::time::SystemTime;

use axum::http::HeaderMap;

pub struct Cabeceras(pub HeaderMap);

impl Cabeceras {
    pub fn agente_valido(&self) -> bool {
        let cabecera_agente = self.0.get("user-agent");
        let agente;
        match cabecera_agente {
            None => return false,
            Some(ok) => agente = ok,
        }
        let valor_agente;
        match agente.to_str() {
            Err(_) => return false,
            Ok(ok) => valor_agente = ok,
        }
        valor_agente.starts_with("Mozilla")
    }
    pub fn supera_numero_minimo(&self, cifra: usize) -> bool {
        self.0.len() >= cifra
    }
}

pub fn obtener_tiempo_unix() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("no se ha podido obtener el systemTime"),
    }
}
