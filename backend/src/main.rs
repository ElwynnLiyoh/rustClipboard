use clipboard::model::*;
use clipboard::service;
use rocket::serde::json::Json;
use simplelog::*;
use std::fs::File;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    // init logger
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("info.log").unwrap(),
    )])
    .unwrap();

    info!("starting server...");
    rocket::build()
        .mount("/", routes![hello])
        .mount("/get", routes![get_empty, get])
        .mount("/upload", routes![upload])
}

/// using for health check
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, Rocket!🚀\nThis is a online clipboard Rust web application using Rocket framework."
}

#[get("/")]
fn get_empty() -> Json<ClipboardResponse> {
    return Json(ClipboardResponse::failed(
        "Code cannot be empty".to_string(),
    ));
}

#[get("/<code>")]
fn get(code: &str) -> Json<ClipboardResponse> {
    if code.is_empty() {
        return get_empty();
    }

    // retrieve the text from Redis
    let response = match service::get_text(code) {
        Ok(text) => ClipboardResponse::success(text),
        Err(msg) => ClipboardResponse::failed(msg),
    };
    Json(response)
}

#[post("/", data = "<req>")]
fn upload(mut req: Json<ClipboardRequest<'_>>) -> Json<ClipboardResponse> {
    //  validate the request
    if req.text.is_empty() {
        return Json(ClipboardResponse::failed(
            "Text cannot be empty".to_string(),
        ));
    }
    if req.expire_time == 0 {
        return Json(ClipboardResponse::failed(
            "Expire time must be greater than 0".to_string(),
        ));
    } else {
        req.min2sec();
    }
    if req.access_limit == 0 {
        return Json(ClipboardResponse::failed(
            "Access limit must be greater than 0".to_string(),
        ));
    }

    // save the text to Redis
    let response = match service::save_text(&req) {
        Ok(hash_code) => ClipboardResponse::success(hash_code),
        Err(msg) => ClipboardResponse::failed(msg),
    };
    Json(response)
}
