#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate mount;
extern crate router;
extern crate serde_json;
extern crate staticfile;

use std::io::Read;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

use iron::{Iron, IronResult, Request, Response, status};
use iron::mime::Mime;
use mount::Mount;
use router::Router;
use staticfile::Static;

use agent::Agent;
use connection::{Connection, ConnectionRequest};

mod agent;
mod connection;

lazy_static! {
    static ref JSON_CONTENT: Mime = "application/json".parse().unwrap();
    static ref CONNECTION: Mutex<Connection> = Mutex::new(Connection::new());
}

fn agents(_: &mut Request) -> IronResult<Response> {
    let agents = vec![Agent::new("rawr!", false),
                      Agent::new("meow!", true),
                      Agent::new("cheep!", false),
                      Agent::new("nyan!", false)];
    Ok(Response::with((status::Ok, serde_json::to_string(&agents).unwrap(), JSON_CONTENT.clone())))
}

fn get_connection(_: &mut Request) -> IronResult<Response> {
    let connection = CONNECTION.lock().unwrap();

    Ok(Response::with((status::Ok, serde_json::to_string(&(*connection)).unwrap(), JSON_CONTENT.clone())))
}

fn connect(req: &mut Request) -> IronResult<Response> {
    let mut buf = String::with_capacity(256);

    req.body.read_to_string(&mut buf).unwrap();
    let value: ConnectionRequest = serde_json::from_str(&buf).unwrap();

    // Actually connect

    match CONNECTION.lock().unwrap().connect(IpAddr::from_str(&value.url).unwrap()) {
        Ok(_) => Ok(Response::with((status::Ok))),
        Err(why) => Ok(Response::with((status::BadRequest, why)))
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/agents", agents)
        .get("/connection", get_connection)
        .put("/connection", connect);

    let mut mount = Mount::new();
    mount.mount("/api", router)
        .mount("/", Static::new(Path::new("public/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
