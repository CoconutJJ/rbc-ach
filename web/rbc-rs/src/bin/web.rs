use std::process::exit;

use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use futures::{future, StreamExt, TryStreamExt};
use open::that;
use serde::Deserialize;

#[path = "../lib/mod.rs"]
mod lib;
use lib::types::RecordType;

#[path = "../csvconv/mod.rs"]
mod csvconv;
use csvconv::csv::convert_to_cpa005;

#[derive(Deserialize)]
struct ConvertRequestQuery {
    convtype: String,
}

#[post("/convert")]
async fn convert(mut body: Multipart, q: web::Query<ConvertRequestQuery>) -> HttpResponse {
    let mut file_data = String::new();
    let mut file_name = String::new();
    while let Ok(Some(mut p)) = body.try_next().await {
        file_name = p.content_disposition().get_filename().unwrap().to_string();
        while let Some(chunk) = p.next().await {
            let chunk = chunk.unwrap();
            file_data.push_str(&String::from_utf8_lossy(chunk.as_ref()));
        }
    }

    let cpa_format = match q.convtype.trim() {
        "PDS" => convert_to_cpa005(file_data, RecordType::Credit),
        "PAD" => convert_to_cpa005(file_data, RecordType::Debit),
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };

    match cpa_format {
        Ok(s) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .insert_header(ContentDisposition::attachment(file_name))
            .body(s),
        Err(log) => HttpResponse::BadRequest()
            .content_type(ContentType::plaintext())
            .body(log.to_string()),
    }
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../../ui/dist/index.html"))
}

async fn start_client() {
    match open::that("http://localhost:8080") {
        Ok(_) => (),
        Err(_) => exit(1),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(index).service(convert))
        .bind(("0.0.0.0", 8080))?
        .run();

    let (result, _) = future::join(server, start_client()).await;

    if result.is_err() {
        exit(1);
    }

    return Ok(());
}
