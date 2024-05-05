use std::sync::Arc;

use axum::{
    extract::{Form, Path, State},
    http::{HeaderMap, Response, StatusCode},
};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::Deserialize;

use crate::{
    utilidades::{self, Cabeceras},
    App,
};

#[derive(Deserialize)]
pub struct Autentificacion {
    usuario: String,
    password: String,
}

fn generar_cookie_autentificacion(magic_crypt_app: &MagicCrypt256) -> String {
    let contenido = format!("sesion_autentificada_{}", utilidades::obtener_tiempo_unix());
    magic_crypt_app.encrypt_str_to_base64(contenido)
}

fn comprobar_cookie_autentificacion(magic_crypt_app: &MagicCrypt256, encriptado: &str) -> bool {
    match magic_crypt_app.decrypt_base64_to_string(encriptado) {
        Err(_) => return false,
        Ok(ok) => {
            if !ok.starts_with("sesion_autentificada_") {
                return false;
            }
            let valor = ok.strip_prefix("sesion_autentificada_").unwrap();
            match valor.parse::<i64>() {
                Err(_) => false,
                Ok(num) => {
                    if num > 1707903301 {
                        return true;
                    }
                    false
                }
            }
        }
    }
}

pub async fn autentificar(
    State(estado): State<Arc<App>>,
    Form(formulario): Form<Autentificacion>,
) -> Response<String> {
    if formulario.usuario != "ninja" || formulario.password != "Scraper13%" {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("autentificación incorrecta".to_string())
            .unwrap();
    }

    let cookie_autentificacion = generar_cookie_autentificacion(&estado.magic_crypt_app);
    let cabecera_cookie = format!("sesion={cookie_autentificacion}");
    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Set-Cookie", cabecera_cookie)
        .header("Location", "/html/reto05/generos")
        .body("usuario autentificado".to_string())
        .unwrap();
}

pub async fn servir_pagina(
    State(estado): State<Arc<App>>,
    Path(pagina): Path<String>,
    cabeceras: HeaderMap,
) -> Response<String> {
    // Comprobaciones agente
    let c = Cabeceras(cabeceras);
    if !c.agente_valido() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("bad requests".to_string())
            .unwrap();
    }
    if !c.supera_numero_minimo(6) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("bad requests".to_string())
            .unwrap();
    }

    // Comprobar si está autentificado
    let mut autentificacion_valida = false;
    match c.0.get("cookie") {
        None => (),
        Some(ok) => {
            let v = ok.to_str();
            if v.is_ok() {
                let cookie = v.unwrap();
                if cookie.starts_with("sesion=") {
                    let comprobar_valor = cookie.strip_prefix("sesion=");
                    if comprobar_valor.is_some() {
                        let encriptado = comprobar_valor.unwrap();
                        autentificacion_valida =
                            comprobar_cookie_autentificacion(&estado.magic_crypt_app, encriptado)
                    }
                }
            }
        }
    }

    match pagina.as_str() {
        "login" => {
            if autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header("Location", "/html/reto05/generos")
                    .body("redirección a géneros".to_string())
                    .unwrap();
            }
            let login = include_str!("reto05/login.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(login.to_string())
                .unwrap();
        }
        "logout" => {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header("Set-Cookie", "sesion=false")
                .header("Location", "/html/reto05/login")
                .body("logout".to_string())
                .unwrap();
        }
        "generos" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let generos = include_str!("reto05/generos.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(generos.to_string())
                .unwrap();
        }

        "10_code" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let ten_code = include_str!("reto05/10_code.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(ten_code.to_string())
                .unwrap();
        }

        "abrams" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let abrams = include_str!("reto05/abrams.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(abrams.to_string())
                .unwrap();
        }

        "ace_of_base" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let ace_of_base = include_str!("reto05/ace_of_base.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(ace_of_base.to_string())
                .unwrap();
        }

        "airplay" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let airplay = include_str!("reto05/airplay.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(airplay.to_string())
                .unwrap();
        }

        "alizbar" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let alizbar = include_str!("reto05/alizbar.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(alizbar.to_string())
                .unwrap();
        }

        "allison" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let allison = include_str!("reto05/allison.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(allison.to_string())
                .unwrap();
        }

        "annie_taylor" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let annie_taylor = include_str!("reto05/annie_taylor.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(annie_taylor.to_string())
                .unwrap();
        }

        "annwn" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let annwn = include_str!("reto05/annwn.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(annwn.to_string())
                .unwrap();
        }

        "antischism" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let antischism = include_str!("reto05/antischism.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(antischism.to_string())
                .unwrap();
        }

        "audioslave" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let audioslave = include_str!("reto05/audioslave.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(audioslave.to_string())
                .unwrap();
        }

        "avishai_cohen" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let avishai_cohen = include_str!("reto05/avishai_cohen.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(avishai_cohen.to_string())
                .unwrap();
        }

        "bad_english" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let bad_english = include_str!("reto05/bad_english.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(bad_english.to_string())
                .unwrap();
        }

        "battle_of_disarm" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let battle_of_disarm = include_str!("reto05/battle_of_disarm.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(battle_of_disarm.to_string())
                .unwrap();
        }

        "beau_and_the_arrows" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let beau_and_the_arrows = include_str!("reto05/beau_and_the_arrows.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(beau_and_the_arrows.to_string())
                .unwrap();
        }

        "berliner_philharmoniker" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let berliner_philharmoniker = include_str!("reto05/berliner_philharmoniker.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(berliner_philharmoniker.to_string())
                .unwrap();
        }

        "big_daddy_kane" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let big_daddy_kane = include_str!("reto05/big_daddy_kane.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(big_daddy_kane.to_string())
                .unwrap();
        }

        "big_l" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let big_l = include_str!("reto05/big_l.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(big_l.to_string())
                .unwrap();
        }

        "bill_evans" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let bill_evans = include_str!("reto05/bill_evans.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(bill_evans.to_string())
                .unwrap();
        }

        "bing_crosby" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let bing_crosby = include_str!("reto05/bing_crosby.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(bing_crosby.to_string())
                .unwrap();
        }

        "blaze_bayley" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let blaze_bayley = include_str!("reto05/blaze_bayley.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(blaze_bayley.to_string())
                .unwrap();
        }

        "boogie_down_productions" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let boogie_down_productions = include_str!("reto05/boogie_down_productions.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(boogie_down_productions.to_string())
                .unwrap();
        }

        "booze_brothers" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let booze_brothers = include_str!("reto05/booze_brothers.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(booze_brothers.to_string())
                .unwrap();
        }

        "bryan_adams" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let bryan_adams = include_str!("reto05/bryan_adams.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(bryan_adams.to_string())
                .unwrap();
        }

        "budgie" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let budgie = include_str!("reto05/budgie.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(budgie.to_string())
                .unwrap();
        }

        "candido" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let candido = include_str!("reto05/candido.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(candido.to_string())
                .unwrap();
        }

        "cannonball_adderley" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let cannonball_adderley = include_str!("reto05/cannonball_adderley.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(cannonball_adderley.to_string())
                .unwrap();
        }

        "cappadonna" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let cappadonna = include_str!("reto05/cappadonna.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(cappadonna.to_string())
                .unwrap();
        }

        "carl_philipp_emanuel_bach" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let carl_philipp_emanuel_bach = include_str!("reto05/carl_philipp_emanuel_bach.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(carl_philipp_emanuel_bach.to_string())
                .unwrap();
        }

        "charlie_parker" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let charlie_parker = include_str!("reto05/charlie_parker.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(charlie_parker.to_string())
                .unwrap();
        }

        "chick_corea" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let chick_corea = include_str!("reto05/chick_corea.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(chick_corea.to_string())
                .unwrap();
        }

        "conflict" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let conflict = include_str!("reto05/conflict.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(conflict.to_string())
                .unwrap();
        }

        "corrosion_of_conformity" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let corrosion_of_conformity = include_str!("reto05/corrosion_of_conformity.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(corrosion_of_conformity.to_string())
                .unwrap();
        }

        "dave_brubeck" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let dave_brubeck = include_str!("reto05/dave_brubeck.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(dave_brubeck.to_string())
                .unwrap();
        }

        "dead_horse" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let dead_horse = include_str!("reto05/dead_horse.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(dead_horse.to_string())
                .unwrap();
        }

        "diamond_head" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let diamond_head = include_str!("reto05/diamond_head.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(diamond_head.to_string())
                .unwrap();
        }

        "diddy" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let diddy = include_str!("reto05/diddy.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(diddy.to_string())
                .unwrap();
        }

        "die_fantastischen_vier" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let die_fantastischen_vier = include_str!("reto05/die_fantastischen_vier.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(die_fantastischen_vier.to_string())
                .unwrap();
        }

        "digital_underground" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let digital_underground = include_str!("reto05/digital_underground.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(digital_underground.to_string())
                .unwrap();
        }

        "dj_khaled" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let dj_khaled = include_str!("reto05/dj_khaled.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(dj_khaled.to_string())
                .unwrap();
        }

        "donald_byrd" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let donald_byrd = include_str!("reto05/donald_byrd.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(donald_byrd.to_string())
                .unwrap();
        }

        "dua_lipa" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let dua_lipa = include_str!("reto05/dua_lipa.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(dua_lipa.to_string())
                .unwrap();
        }

        "eddie_palmieri" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let eddie_palmieri = include_str!("reto05/eddie_palmieri.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(eddie_palmieri.to_string())
                .unwrap();
        }

        "elektradrive" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let elektradrive = include_str!("reto05/elektradrive.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(elektradrive.to_string())
                .unwrap();
        }

        "elvis_presley" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let elvis_presley = include_str!("reto05/elvis_presley.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(elvis_presley.to_string())
                .unwrap();
        }

        "etta_james" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let etta_james = include_str!("reto05/etta_james.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(etta_james.to_string())
                .unwrap();
        }

        "falco" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let falco = include_str!("reto05/falco.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(falco.to_string())
                .unwrap();
        }

        "fay_claassen" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let fay_claassen = include_str!("reto05/fay_claassen.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(fay_claassen.to_string())
                .unwrap();
        }

        "five_finger_death_punch" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let five_finger_death_punch = include_str!("reto05/five_finger_death_punch.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(five_finger_death_punch.to_string())
                .unwrap();
        }

        "foreigner" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let foreigner = include_str!("reto05/foreigner.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(foreigner.to_string())
                .unwrap();
        }

        "fryderyk_chopin" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let fryderyk_chopin = include_str!("reto05/fryderyk_chopin.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(fryderyk_chopin.to_string())
                .unwrap();
        }

        "fuzz_forward" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let fuzz_forward = include_str!("reto05/fuzz_forward.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(fuzz_forward.to_string())
                .unwrap();
        }

        "georg_philipp_telemann" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let georg_philipp_telemann = include_str!("reto05/georg_philipp_telemann.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(georg_philipp_telemann.to_string())
                .unwrap();
        }

        "get_rekt" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let get_rekt = include_str!("reto05/get_rekt.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(get_rekt.to_string())
                .unwrap();
        }

        "giuffria" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let giuffria = include_str!("reto05/giuffria.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(giuffria.to_string())
                .unwrap();
        }

        "gloria_estefan" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let gloria_estefan = include_str!("reto05/gloria_estefan.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(gloria_estefan.to_string())
                .unwrap();
        }

        "gost" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let gost = include_str!("reto05/gost.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(gost.to_string())
                .unwrap();
        }

        "hatewanx" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let hatewanx = include_str!("reto05/hatewanx.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(hatewanx.to_string())
                .unwrap();
        }

        "herbert_von_karajan" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let herbert_von_karajan = include_str!("reto05/herbert_von_karajan.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(herbert_von_karajan.to_string())
                .unwrap();
        }

        "hole" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let hole = include_str!("reto05/hole.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(hole.to_string())
                .unwrap();
        }

        "house_of_pain" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let house_of_pain = include_str!("reto05/house_of_pain.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(house_of_pain.to_string())
                .unwrap();
        }

        "iron_reagan" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let iron_reagan = include_str!("reto05/iron_reagan.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(iron_reagan.to_string())
                .unwrap();
        }

        "johannes_brahms" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let johannes_brahms = include_str!("reto05/johannes_brahms.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(johannes_brahms.to_string())
                .unwrap();
        }

        "johann_pachelbel" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let johann_pachelbel = include_str!("reto05/johann_pachelbel.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(johann_pachelbel.to_string())
                .unwrap();
        }

        "johann_strauss" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let johann_strauss = include_str!("reto05/johann_strauss.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(johann_strauss.to_string())
                .unwrap();
        }

        "john_lennon" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let john_lennon = include_str!("reto05/john_lennon.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(john_lennon.to_string())
                .unwrap();
        }

        "jordi_savall" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let jordi_savall = include_str!("reto05/jordi_savall.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(jordi_savall.to_string())
                .unwrap();
        }

        "junior_murvin" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let junior_murvin = include_str!("reto05/junior_murvin.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(junior_murvin.to_string())
                .unwrap();
        }

        "katherine_jenkins" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let katherine_jenkins = include_str!("reto05/katherine_jenkins.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(katherine_jenkins.to_string())
                .unwrap();
        }

        "keith_jarrett" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let keith_jarrett = include_str!("reto05/keith_jarrett.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(keith_jarrett.to_string())
                .unwrap();
        }

        "killers" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let killers = include_str!("reto05/killers.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(killers.to_string())
                .unwrap();
        }

        "kool_g_rap" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let kool_g_rap = include_str!("reto05/kool_g_rap.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(kool_g_rap.to_string())
                .unwrap();
        }

        "krayzie_bone" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let krayzie_bone = include_str!("reto05/krayzie_bone.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(krayzie_bone.to_string())
                .unwrap();
        }

        "l7" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let l7 = include_str!("reto05/l7.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(l7.to_string())
                .unwrap();
        }

        "lady_gaga" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let lady_gaga = include_str!("reto05/lady_gaga.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(lady_gaga.to_string())
                .unwrap();
        }

        "leeway" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let leeway = include_str!("reto05/leeway.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(leeway.to_string())
                .unwrap();
        }

        "lou_donaldson" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let lou_donaldson = include_str!("reto05/lou_donaldson.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(lou_donaldson.to_string())
                .unwrap();
        }

        "louis_armstrong" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let louis_armstrong = include_str!("reto05/louis_armstrong.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(louis_armstrong.to_string())
                .unwrap();
        }

        "machine_head" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let machine_head = include_str!("reto05/machine_head.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(machine_head.to_string())
                .unwrap();
        }

        "marina" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let marina = include_str!("reto05/marina.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(marina.to_string())
                .unwrap();
        }

        "max_richter" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let max_richter = include_str!("reto05/max_richter.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(max_richter.to_string())
                .unwrap();
        }

        "metal_church" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let metal_church = include_str!("reto05/metal_church.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(metal_church.to_string())
                .unwrap();
        }

        "mose_allison" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let mose_allison = include_str!("reto05/mose_allison.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(mose_allison.to_string())
                .unwrap();
        }

        "mustasch" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let mustasch = include_str!("reto05/mustasch.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(mustasch.to_string())
                .unwrap();
        }

        "natalie_imbruglia" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let natalie_imbruglia = include_str!("reto05/natalie_imbruglia.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(natalie_imbruglia.to_string())
                .unwrap();
        }

        "nirvana" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let nirvana = include_str!("reto05/nirvana.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(nirvana.to_string())
                .unwrap();
        }

        "no_alibi" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let no_alibi = include_str!("reto05/no_alibi.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(no_alibi.to_string())
                .unwrap();
        }

        "nonpalidece" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let nonpalidece = include_str!("reto05/nonpalidece.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(nonpalidece.to_string())
                .unwrap();
        }

        "only_living_witness" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let only_living_witness = include_str!("reto05/only_living_witness.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(only_living_witness.to_string())
                .unwrap();
        }

        "ozzy_osbourne" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let ozzy_osbourne = include_str!("reto05/ozzy_osbourne.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(ozzy_osbourne.to_string())
                .unwrap();
        }

        "pantera" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let pantera = include_str!("reto05/pantera.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(pantera.to_string())
                .unwrap();
        }

        "part_1" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let part_1 = include_str!("reto05/part_1.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(part_1.to_string())
                .unwrap();
        }

        "patrick_hawes" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let patrick_hawes = include_str!("reto05/patrick_hawes.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(patrick_hawes.to_string())
                .unwrap();
        }

        "peter_gregson" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let peter_gregson = include_str!("reto05/peter_gregson.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(peter_gregson.to_string())
                .unwrap();
        }

        "pet_shop_boys" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let pet_shop_boys = include_str!("reto05/pet_shop_boys.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(pet_shop_boys.to_string())
                .unwrap();
        }

        "red_fang" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let red_fang = include_str!("reto05/red_fang.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(red_fang.to_string())
                .unwrap();
        }

        "robbie_williams" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let robbie_williams = include_str!("reto05/robbie_williams.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(robbie_williams.to_string())
                .unwrap();
        }

        "sarah_connor" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let sarah_connor = include_str!("reto05/sarah_connor.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(sarah_connor.to_string())
                .unwrap();
        }

        "sarah_vaughan" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let sarah_vaughan = include_str!("reto05/sarah_vaughan.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(sarah_vaughan.to_string())
                .unwrap();
        }

        "savage_garden" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let savage_garden = include_str!("reto05/savage_garden.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(savage_garden.to_string())
                .unwrap();
        }

        "saxon" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let saxon = include_str!("reto05/saxon.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(saxon.to_string())
                .unwrap();
        }

        "sb19" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let sb19 = include_str!("reto05/sb19.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(sb19.to_string())
                .unwrap();
        }

        "scorpions" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let scorpions = include_str!("reto05/scorpions.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(scorpions.to_string())
                .unwrap();
        }

        "screaming_trees" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let screaming_trees = include_str!("reto05/screaming_trees.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(screaming_trees.to_string())
                .unwrap();
        }

        "sepultura" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let sepultura = include_str!("reto05/sepultura.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(sepultura.to_string())
                .unwrap();
        }

        "sierra" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let sierra = include_str!("reto05/sierra.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(sierra.to_string())
                .unwrap();
        }

        "stanley_clarke" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let stanley_clarke = include_str!("reto05/stanley_clarke.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(stanley_clarke.to_string())
                .unwrap();
        }

        "stryper" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let stryper = include_str!("reto05/stryper.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(stryper.to_string())
                .unwrap();
        }

        "styx" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let styx = include_str!("reto05/styx.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(styx.to_string())
                .unwrap();
        }

        "the_avalanches" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_avalanches = include_str!("reto05/the_avalanches.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_avalanches.to_string())
                .unwrap();
        }

        "the_beach_boys" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_beach_boys = include_str!("reto05/the_beach_boys.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_beach_boys.to_string())
                .unwrap();
        }

        "the_bluestone" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_bluestone = include_str!("reto05/the_bluestone.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_bluestone.to_string())
                .unwrap();
        }

        "the_exploited" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_exploited = include_str!("reto05/the_exploited.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_exploited.to_string())
                .unwrap();
        }

        "the_pharcyde" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_pharcyde = include_str!("reto05/the_pharcyde.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_pharcyde.to_string())
                .unwrap();
        }

        "the_rocking_dildos" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let the_rocking_dildos = include_str!("reto05/the_rocking_dildos.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(the_rocking_dildos.to_string())
                .unwrap();
        }

        "tony_williams" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let tony_williams = include_str!("reto05/tony_williams.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(tony_williams.to_string())
                .unwrap();
        }

        "wyclef_jean" => {
            if !autentificacion_valida {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("acceso prohibido".to_string())
                    .unwrap();
            }
            let wyclef_jean = include_str!("reto05/wyclef_jean.html");
            return Response::builder()
                .status(StatusCode::OK)
                .body(wyclef_jean.to_string())
                .unwrap();
        }

        _ => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("not found".to_string())
                .unwrap();
        }
    }
}
