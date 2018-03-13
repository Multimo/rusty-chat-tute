extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::server::{Request, Response, Service};

use futures::future::Future;

struct Microservice;

const PHRASE: &'static str = "Hello, World!";

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {

        Box::new(futures::future::ok(
            Response::new()
                .with_body(PHRASE)
        ))
    }
}


fn main() {
    print!("running");
    env_logger::init();
    let address = "127.0.0.1:8888".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}

// fn call(&self, request: Request) -> Self::Future {
//         match (request.method(), request.path()) {
//             (&Post, "/") => {
//                 let future = request
//                     .body()
//                     .concat2()
//                     .and_then(parse_form)
//                     .and_then(write_to_db)
//                     .then(make_post_response);
//                 Box::new(future)
//             }
//             _ => Box::new(futures::future::ok(
//                 Response::new().with_status(StatusCode::NotFound),
//             )),
//         }
//     }