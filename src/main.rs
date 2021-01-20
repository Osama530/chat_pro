#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
// extern crate rocket_auth_login as auth;

use rocket::request::{FlashMessage, Form, FromRequest};
// use auth::authorization;
use rocket_contrib::json::{Json, JsonValue};

mod administrator;
use administrator::*;

// use administrator::AdministratorCookies;

const URL: &'static str = "http://localhost:8000";
const LOGIN_URL: &'static str = "http://localhost:8000/login";

#[get("/")]
fn index(admin_opt: Option<AdministratorCookie>, flash_msg_opt: Option<FlashMessage>)-> &'static str {
    let mut content = String::with_capacity(200);
    if let Some(flash) = flash_msg_opt {
        match flash.name() {
            "success" => content.push_str(flash.msg()),
            "warning" => content.push_str(flash.msg()),
	        "error" => content.push_str(flash.msg()),
            _ => content.push_str(flash.msg()),
        }
    } 
    if let Some(admin) = admin_opt {
        content.push_str(&format!("Welcome {}", admin.username));
    }
    else {
        content.push_str(r#"<a href="/login">Login</a>"#);
    }

    &content
}

#[get("/login")]
fn logged_in()-> String {
    "you are succsesfully logged in".to_string()
    // json!({
    //     "success": true,
    // })
}

fn main() {
    rocket::ignite()
	        
    // If using a database connection:
    // .manage(data::init_pg_pool())
    
    // using rocket_contrib's Templates
    // .attach(Template::fairing())
    .mount("/", routes![
        index,
        logged_in
    ])
    .launch();

}