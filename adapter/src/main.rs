#[macro_use]
extern crate ini;

extern crate reqwest;
extern crate rand;
extern crate actix_web;
mod common;
use actix_web::{web, App, HttpServer}; //middleware

use common::{load_config_data,adapterhandler, AppState};

#[actix_web::main]
  async fn main() -> std::io::Result<()> {
    println!("Server Starting"); 

    let appdata: AppState = match load_config_data() {
        Ok(appdata) => appdata,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    

    let confidata = web::Data::new(appdata);

    println!("Server Started");
    HttpServer::new(move || {
      App::new()
       // .wrap(middleware::Compress::default())
        .app_data(confidata.clone())
        .route("/api/adapter/supplier", actix_web::web::get().to(adapterhandler))
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
  }
