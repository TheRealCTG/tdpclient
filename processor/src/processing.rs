

 use actix_web::{web, HttpResponse,Responder};
 use anyhow::Result;
// use std::time::Instant;
use applib::xmlutil::extract_nodes_from_xml;
use crate::common::{AppState, simulate_cpu_usage};
use rand::{thread_rng, Rng};
use async_std::sync::Arc;
use async_std::sync::Mutex;
use futures::future;
use crate::definitions::ProcessorCallError;
use std::sync::atomic::{AtomicUsize, Ordering};


pub async fn get_accomodation_handler(data: web::Data<AppState>) -> impl Responder {

    
    let configdata = data.get_ref().clone();


    match get_accomodations(configdata).await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => {
            
            println!("{}", err);
            HttpResponse::InternalServerError().body("Error in processing request")
        } 
    }
 }



 async fn  get_accomodations(configdata: AppState) -> Result<String, ProcessorCallError> {
    //let now = Instant::now();  
   
    let supplier_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let max_noofsuppliers_forrandomness = configdata.max_noofsuppliers_forrandomness.to_string();
    let min_noofsuppliers_forrandomness = configdata.min_noofsuppliers_forrandomness.to_string();
    // //convert string to integer
    let imax_rand: u32 = max_noofsuppliers_forrandomness.parse::<u32>().unwrap();
    let imin_rand: u32 = min_noofsuppliers_forrandomness.parse::<u32>().unwrap();

    let mut rng = thread_rng();
    let  max_no_of_supplier: u32 = rng.gen_range(imin_rand..imax_rand);

    let mut supplier_from_index = rng.gen_range(0..supplier_list.len());
   
    let mut suppliers: Vec<i32> = Vec::new();   
    
    for _i in 0..max_no_of_supplier {
        suppliers.push(supplier_list[supplier_from_index]);
        supplier_from_index = (supplier_from_index + 1) % supplier_list.len();
    }
  
    let mut handles = Vec::new();        
  
    let result =  Arc::new(Mutex::new(String::with_capacity(50000)));
    let counter = Arc::new(AtomicUsize::new(0));

   // println!("get_accomodations called=>enumerate suppliers {}",  suppliers.len());  
    //let mut  i = 0;
    for  supplier in suppliers.iter(){      
      
         let resultc = Arc::clone(&result);
         let counter = Arc::clone(&counter);
         let hostname =  configdata.adaptorhosturl.clone();
         let isupplierdata = supplier.clone();         
         let handle = tokio::task::spawn(async move {          
             let data = match  get_accommodation_by_supplier(isupplierdata,hostname.clone()).await {
                 Ok(data) => {
                    data
                 },
                 Err(error) => { 
                     println!("{}", error);
                     return; 
                 }
             };
            let res = match extract_nodes_from_xml(data.to_string(), "Hotel".into()).await {
                Ok(res) => res,
                Err(error) => {
                    println!("{}", error);
                    return; 
                },
              };   
            let strval = match String::from_utf8(res.0)
            {
                Ok(strval) => strval,
                Err(error) => {
                                println!("{}", error);
                                return; 
                }
            };
            let mut result = resultc.lock().await;   
            result.push_str(strval.as_str());          
            counter.fetch_add(res.1, Ordering::Relaxed);
                   
             
         });   
       
          handles.push(handle);
    }

    future::join_all(handles).await;
    
    
    let  value  =   counter.load(Ordering::Relaxed);
    let value_string = value.to_string();
    let str = "<HotelFindResponse time=\"0.21500015258789\" ipaddress=\"14.140.153.130\" count=\"".to_owned(); 
    let str2 = "\">\r\n    <ArrivalDate>01/06/2024</ArrivalDate>\r\n    <DepartureDate>10/06/2024</DepartureDate>\r\n    <Currency>INR</Currency>\r\n    <GuestNationality>IN</GuestNationality>\r\n    <SearchSessionId>17168872488751716887248949665</SearchSessionId><Hotels>";
    let mut resultdata = str  + value_string.as_str() + str2;
   
    let resdata = result.lock().await;
    resultdata.push_str(resdata.as_str());
    resultdata.push_str("\r\n</Hotels>\r\n</HotelFindResponse>");
   
    simulate_cpu_usage(resultdata.clone(), configdata); 
   Ok(resultdata.to_string())
}
 //
 pub async fn get_accommodation_by_supplier(supplier_id: i32,hostname:String) -> Result<String, ProcessorCallError>  {
    let url = format!("{0}{1}",hostname, supplier_id);
    //println!("URL: {}", url);
  
    let body = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(body) => body,
            Err(_err) => return Err(ProcessorCallError::Error("Error in getting response from adaptor".to_string()))
        },
        Err(_err) => return Err(ProcessorCallError::Error("Error in getting response from adaptor".to_string()))
    };   


 
  Ok(body.to_string())

}

