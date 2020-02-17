#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

#[get("/hello/<name>")]
fn hello(name: String) -> String
{
    format!("Hello, {}!", name.as_str())
}

#[get("/roly?poly&<song>")]
fn roly(song: &RawStr) -> String
{   // /roly?song=T-TA&poly
    format!("I am {}!", song.as_str())
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/",
        routes![
            index,
            hello,
            roly
        ]
    ).launch();
}