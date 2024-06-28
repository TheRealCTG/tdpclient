#[macro_use]
extern crate ini;
extern crate futures;
extern crate rand;
extern crate reqwest;
extern crate quick_xml;
extern crate serde;
use actix_web::{web, HttpServer,App};

mod definitions;
 mod processing;
 mod common;
 use common::{load_config_data, AppState};
 use processing::get_accomodation_handler;

//#[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {


    let appdata: AppState = match load_config_data() {
        Ok(appdata) => appdata,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    

    let confidata = web::Data::new(appdata);
    println!("Starting the server");
    HttpServer::new(move || {
        App::new()
            .app_data(confidata.clone())
            .route("/api/get-accomodations/", web::get().to(get_accomodation_handler))
    }).bind(("0.0.0.0", 8090))?
    .run()
    .await

}















