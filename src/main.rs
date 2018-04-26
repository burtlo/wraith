#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
use dotenv::dotenv;
use std::env;

extern crate rocket;
use rocket_contrib::{Json, Value};

#[macro_use] extern crate diesel;
use diesel::prelude::*;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod schema;
use schema::scans;
mod scan;
use scan::{Scan};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> Json<Value> {
    let ordered_scans = scans::table.order(scans::id.asc())
        .load::<Scan>(&establish_connection())
        .unwrap();
  
    Json(json!(ordered_scans))
}

#[get("/<id>")]
fn read(id: i32) -> Json<Value> {
    let existing_scans = scans::table.filter(scans::id.eq(id)).load::<Scan>(&establish_connection());
    
    match existing_scans {
        Ok(found_scans) => {
            if found_scans.len() == 1 {
                Json(json!(found_scans[0]))    
            } else {
                Json(json!({ "status" : "failed", "message" : format!("Could not find scan with index {}",id) }))
            }
            
        },
        Err(message) => {
            Json(json!({ "status" : "failed", "message" : format!("{}",message) }))
        }
    }
}

#[post("/", data="<scan>")]
fn create(scan: Json<Scan>) -> Json<Value> {
    let insert = Scan { id: None, ..scan.into_inner() };
    let created_scan = Scan::create(insert, &establish_connection());
    Json(json!(created_scan))
}


#[put("/<id>", data = "<scan>")]
fn update(id: i32, scan: Json<Scan>) -> Json<Value> {
    let updated_scan = Scan { id: Some(id), ..scan.into_inner() };
    Json(json!({ "status" : Scan::update(id, updated_scan, &establish_connection()) }))
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    let result = diesel::delete(scans::table.find(id)).execute(&establish_connection());

    match result {
        Ok(val) => {
            Json(json!({"status" : "ok"}))
        },
        Err(message) => {
            Json(json!({ "status" : "failed", "message" : format!("{}",message) }))
        }
    }
}

#[get("/")]
fn hello() -> String {
    format!("Hello, friend.")
}

fn scan_exists(id: i32) -> Option<i32> {
    let existing_scans = scans::table.filter(scans::id.eq(id)).load::<Scan>(&establish_connection());

    match existing_scans {
        Ok(found_scans) => {
            if found_scans.len() == 1 {
                Some(id)
            } else {
                None
            }
        },
        Err(message) => {
            println!("Error querying the database, which does not give me much hope for the code ahead!\n");
            println!("{}",message);
            None
        }
    }
}

fn main() {

    // Before launching the web service, seed the database with scans if they are not already present.
    // 
    // NOTE: This could be done in the migration but it was a useful exercise to learn how 
    //   perform the query and work with the results.

    // Search for a scan with the id of 1, 
    // if the query is successful and the number of scans found is 0, then go ahead an insert the seed data
    // if there was an error display a message about the dire future ahead.

    match scan_exists(1) {
        Some(id) => {
            println!("DATABASE - Example scans already present.");
        },
        None => {
            println!("DATABASE - Seeding with example scan.");

            let new_scan = Scan { 
                    id: Some(1), 
                    data: "{ \"scan\" : \"1\" }".to_string() 
                };

            diesel::insert_into(scans::table)
                .values(&new_scan)
                .execute(&establish_connection())
                .expect(&format!("Error saving scan {:?} to the database", new_scan.id));
            }
    }

    // Now all that is left is to start the web server with the various functions
    // we have created mounted to the appropriate paths.

    rocket::ignite()
        .mount("/",routes![hello])
        .mount("/scans", routes![index, create, read, update, delete])
        .launch();

}