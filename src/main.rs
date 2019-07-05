#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod routes;
mod graphql;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::hello::say_hello, routes::posts::get_posts])
        .launch();
}
