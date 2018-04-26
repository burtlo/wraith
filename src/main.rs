#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate dotenv;

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};


use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;
mod scan;
use scan::{Scan};
use schema::scans;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[post("/", data="<scan>")]
fn create(scan: Json<Scan>) -> Json<Scan> {
    scan
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "scan 1", 
        "scan 2"
    ]))    
}

#[put("/<id>", data = "<scan>")]
fn update(id: i32, scan: Json<Scan>) -> Json<Scan> {
    scan
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status" : "ok"}))
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!",age, name)
}

fn main() {
    rocket::ignite()
        .mount("/scans", routes![create, read, update, delete])
        .mount("/hello",routes![hello])
        .launch();



    let scan_one = Scan {
        id: 1,
        body: "{ \"scan\" : \"1\" }"
    }

    diesel::insert_into(scans::table)
        .values(&scan_one)
        .execute(establish_connection())
        .expect("Error saving example description");
    
}