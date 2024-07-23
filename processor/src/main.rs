#[macro_use]
extern crate ini;
extern crate futures;
extern crate rand;
extern crate reqwest;
extern crate quick_xml;
extern crate serde;
extern crate uuid;
extern crate hex;
use actix_web::{web, HttpServer,App};//, middleware};

mod definitions;
 mod processing;
 mod common;
 use common::{load_config_data, AppState,load_file};
 use processing::get_accomodation_handler;

//#[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {

    
    let filedata = match load_file()
    {
      Ok(x) => x,
      Err(e) => panic!("Error: {}", e),
    };
  
    let appdata: AppState = match load_config_data(filedata[0].clone()) {
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
            //.wrap(middleware::Compress::default())
            .app_data(confidata.clone())
            .route("/api/get-accomodations/", web::get().to(get_accomodation_handler))
    }).bind(("0.0.0.0", 8090))?
    .run()
    .await

}















