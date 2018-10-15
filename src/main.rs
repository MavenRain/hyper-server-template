extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
const PHRASE: & str = "Hello from the Rust server!";

fn main() {
  let address = ([0,0,0,0], 8080).into();
  let service = || {
    service_fn_ok(|_| {
      Response::new(Body::from(PHRASE))
    })
  };
  let server = Server::bind(& address).serve(service).map_err(|e|
    println!("Server error: {}", e));
  rt::run(server);
}
