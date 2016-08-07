#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate mount;
extern crate router;
extern crate serde_json;
extern crate staticfile;

use std::path::Path;

use iron::{Iron, IronResult, Request, Response, status};

use mount::Mount;
use router::Router;
use staticfile::Static;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    active: bool,
    details: String,
}

impl Agent {
    pub fn new(name: &str, active: bool) -> Agent {
        Agent {name: name.to_owned(), active: active, details: format!("Super interesting details for {}", name)}
    }
}

fn agents(_: &mut Request) -> IronResult<Response> {
    let agents = vec![Agent::new("rawr!", false), Agent::new("meow!", true), Agent::new("cheep!", false), Agent::new("nyan!", false)];
    Ok(Response::with((status::Ok, serde_json::to_string(&agents).unwrap())))
}

fn main() {
    let mut router = Router::new();
    router
        .get("/agents", agents);

    let mut mount = Mount::new();
    mount
        .mount("/api", router)
        .mount("/", Static::new(Path::new("public")));

    Iron::new(mount).http("localhost:3000").unwrap();
}

