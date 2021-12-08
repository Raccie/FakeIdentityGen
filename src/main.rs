use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::{fs::File, io::BufReader};
use std::collections::HashMap;
use rand::seq::SliceRandom;

#[get("/")]
async fn hello() -> impl Responder {
    let file1 = File::open("resources/names_eng.json").expect("File not found");
    let file2 = File::open("resources/surnames_usa.json").expect("File not found");
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let prename_json: HashMap<String, Vec<String>> = serde_json::from_reader(reader1).expect("Could not parse JSON");
    let prenames = [&prename_json["boy"][..], &prename_json["girl"][..]].concat();

    let surnames: Vec<String> = serde_json::from_reader(reader2).expect("Could not parse JSON");

    let mut out_v: Vec<String> = Vec::new();

    for _ in 0..1_000_000 {        
        let prename = prenames.choose(&mut rand::thread_rng()).expect("Bruh").as_str();
        let surname = surnames.choose(&mut rand::thread_rng()).expect("Bruh").as_str();
        
        let mut name = "".to_owned();
        name.push_str(prename);
        name.push_str(" ");
        name.push_str(surname);
        out_v.push(name);
    }

    HttpResponse::Ok().json(out_v)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}