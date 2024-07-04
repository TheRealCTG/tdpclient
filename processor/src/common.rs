
// use awc::cookie::time::Instant;
use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};
use uuid::Uuid;
//use std::time::{Instant};
#[derive(Clone)]
pub struct AppState {
    pub min_noofsuppliers_forrandomness: u64,
    pub  max_noofsuppliers_forrandomness: u64,
    pub min_cpu_usage_in_milliseconds: u64,
    pub  max_cpu_usage_in_milliseconds: u64, 
    pub supplierhosturl: String
 }

pub fn load_config_data() -> Result<AppState,&'static str> {

    let app_ini = ini!(r"app.ini");
    let min_noofsuppliers_forrandomness  = match app_ini["section1"]["minnoofsuppliersforrandomness"].as_ref() 
    {
        Some(value) => value.clone(),
        None => return Err("minnoofsuppliers is not found in app.ini")
    };
    let max_noofsuppliers_forrandomness = match app_ini["section1"]["maxnoofsuppliersforrandomness"].as_ref() 
    {
        Some(value) => value.clone(),
        None => return Err("maxnoofsuppliersforrandomness is not found in app.ini")
    };
    let min_cpu_usage_in_milliseconds = match app_ini["section1"]["mincpuusageinmilliseconds"].as_ref()
    {
        Some(value) => value.clone(),
        None => return Err("mincpuusageinmilliseconds is not found in app.ini")
    };
    let max_cpu_usage_in_milliseconds = match app_ini["section1"]["maxcpuusageinmilliseconds"].as_ref()
    {
        Some(value) => value.clone(),
        None => return Err("maxcpuusageinmilliseconds is not found in app.ini")
    };
    let supplierhostname = match app_ini["section1"]["supplierhostname"].as_ref()
    {
        Some(value) => value.clone(),
        None => return Err("supplierhostname is not found in app.ini")
    };

    let supplierport = match app_ini["section1"]["supplierport"].as_ref()
    {
        Some(value) => value.clone(),
        None => return Err("supplierport is not found in app.ini")
    };

    let suppierhohsturl = "http://".to_string() + &supplierhostname+ ":"+
    &supplierport.to_string() +"/api/supplier?supplier_id=";
    //println!("Supplier Host URL: {}", suppierhohsturl.clone());
    // format!("{0}{1}{2}{3}{4}","http://", 
    // supplierhostname.to_string(),":",supplierport.to_string(),  "/api/supplier?supplierId=");
   println!("Supplier Host URL: {}", suppierhohsturl.clone());
   Ok(AppState {
    min_noofsuppliers_forrandomness: min_noofsuppliers_forrandomness.parse::<u64>().unwrap(),
    max_noofsuppliers_forrandomness: max_noofsuppliers_forrandomness.parse::<u64>().unwrap(),
    min_cpu_usage_in_milliseconds: min_cpu_usage_in_milliseconds.parse::<u64>().unwrap(),
    max_cpu_usage_in_milliseconds: max_cpu_usage_in_milliseconds.parse::<u64>().unwrap(),    
    supplierhosturl: suppierhohsturl.clone()
}) 
}




pub fn simulate_cpu_usage(ref xml_document:String, confidata: AppState) {  
    let max_cpu_usage_for_in_milliseconds = confidata.max_cpu_usage_in_milliseconds;
    let min_cpu_usage_for_in_milliseconds = confidata.min_cpu_usage_in_milliseconds;
    let mut rng = thread_rng();
    let delayduration = rng.gen_range(min_cpu_usage_for_in_milliseconds..max_cpu_usage_for_in_milliseconds);

    let loop_till_time = std::time::Instant::now() +  std::time::Duration::from_millis(delayduration);
   
	let mut   loop_counter = 0;
    if xml_document.len() > 0 {
        while std::time::Instant::now() < loop_till_time {           
           let _xml_hash = create_hash(&xml_document);
           loop_counter = loop_counter + 1;
        }
    }
}

fn create_hash(xml_document: &str) -> String {
    let text = format!("{}{}", xml_document, create_new_guid());
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

fn create_new_guid() -> String {
    let new_guid = Uuid::new_v4();
    new_guid.to_string()
}
