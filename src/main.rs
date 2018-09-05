extern crate iron;
extern crate rustc_serialize;

use std::collections::HashMap;

use iron::prelude::*;
use iron::{Handler};
use iron::status;
use iron::headers::ContentType;
use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct Letter {
    title: String,
    message: String
}

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }
    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

fn json(_: &mut Request) -> IronResult<Response> {
    let letter = Letter {
        title: "title...".to_string(),
        message: "message...".to_string()
    };
    let payload = json::encode(&letter).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

fn bad(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::BadRequest))
}

fn main() {
    let mut router = Router::new();

    router.add_route("json".to_string(), json);

    router.add_route("error".to_string(), bad);

    let host = "localhost:3000";
    println!("binding on {}", host);
    Iron::new(router).http(host).unwrap();
}
