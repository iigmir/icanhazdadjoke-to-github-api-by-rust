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
