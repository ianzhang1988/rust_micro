// extern crate hyper;

// use hyper::server::builder;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Server, Response};

fn main() {
    let addr = ([127,0,0,1],8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| service_fn_ok(|_| Response::new(Body::from("hello, world"))));

    let server = server.map_err(drop);
    hyper::rt::run(server)
}
