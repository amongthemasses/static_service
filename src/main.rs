#![feature(proc_macro_hygiene, decl_macro)]
#![feature(str_split_as_str)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod controllers;

use controllers::index;
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use std::path::PathBuf;
use std::env;

struct AssetsDir(String);

pub fn get_cors() -> Cors {
    // You can also deserialize this
    rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        // allowed_origins: AllowedOrigins::some_exact(&["https://www.acme.com"]), // 允许访问的域，这里允许全部，如果要指定其他可以
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        // 指定header：AllowedHeaders::some(&["Authorization", "Accept"]),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("cors config error")
}

fn _routers() {}

fn main() {
    let rocket = rocket::ignite();
    let mut config_path = env::current_dir().unwrap();
    config_path.push("public");
    println!("StaticPath: \n    => {:?}", config_path);
    let rocket = rocket
        .mount("/", StaticFiles::from(config_path.to_str().unwrap()))
        .mount("/apis", routes![index::index_1, index::index_2])
        .attach(get_cors())
        .attach(AdHoc::on_attach("Assets Config", |rocket| {
            let assets_dir = rocket
                .config()
                .get_str("assets_dir")
                .unwrap_or("assets/")
                .to_string();
            Ok(rocket.manage(AssetsDir(assets_dir)))
        }));
    rocket.launch();
}
