#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub mod controller;
pub mod ui;
pub mod color;

use controller::bridge;
use ui::router;

use rocket::fairing::AdHoc;
use rocket::http::Header;

fn main() {
    bridge::init();
    rocket::ignite()
        .attach(AdHoc::on_response(|_, res| {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
        .mount("/api/v1/", router::get_routes())
        .launch();
}
