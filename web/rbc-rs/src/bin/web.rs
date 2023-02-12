use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
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
        "PDS" => convert_to_cpa005(file_data, RecordType::Credit).unwrap(),
        "PAD" => convert_to_cpa005(file_data, RecordType::Debit).unwrap(),
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(ContentDisposition::attachment(file_name))
        .body(cpa_format)
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../../ui/dist/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(convert))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
