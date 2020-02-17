#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket_contrib::json::{ JsonValue };

#[get("/hello/<message>")]
fn hello(message: String) -> JsonValue
{
    return json!({ "message": message });
}

#[get("/")]
fn index() -> &'static str
{
    "Hello, world!"
}

#[catch(404)]
fn not_found() -> JsonValue
{
    return json!({ "status": "error", "reason": "Resource was not found." });
}

#[catch(500)]
fn server_error() -> JsonValue
{
    return json!({ "status": "error", "reason": "Server is down." });
}

fn main()
{
    rocket::ignite()
    .mount("/",routes![ hello, index ])
    .register(catchers![ not_found, server_error ])
    .launch();
}