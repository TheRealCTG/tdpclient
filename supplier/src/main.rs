

extern crate rand;
extern crate actix_web;
use actix_web::{ App,  HttpServer};
use actix_web::web::Data;
mod filehandling;
mod definitions;
use  filehandling::{AppState,load_all_files,get_supplier};

  #[actix_web::main]
  async fn main() -> std::io::Result<()> {
    println!("Server Starting"); 

    let filedata = match load_all_files()
    {
      Ok(x) => x,
      Err(e) => panic!("Error: {}", e),
    };
    let supplier_files  =  Data::new(AppState {
      filecontent: filedata
  });


    HttpServer::new(move || {
      App::new()
        .app_data(supplier_files.clone())
        .route("/api/supplier", actix_web::web::get().to(get_supplier))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
  }


  
 
    



