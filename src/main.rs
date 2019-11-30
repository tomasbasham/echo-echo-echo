#![deny(warnings)]

// A function which runs a future to completion using the Hyper runtime.
use hyper::rt::run;

// Miscellaneous types from Hyper for working with HTTP.
use hyper::{Body, Method, Request, Response, Server, StatusCode};

// This function turns a closure which returns a future into an
// implementation of the the Hyper `Service` trait, which is an asynchronous
// function from a generic `Request` to a `Response`.
use hyper::service::service_fn;

// Extension traits providing additional methods on futures. `FutureExt`
// adds methods that work for all futures, whereas `TryFutureExt` adds
// methods to futures that return `Result` types.
use futures::future::{FutureExt, TryFutureExt};

// Extension trait for futures 0.1 futures, adding the `.compat()` method
// which allows us to use `.await` on 0.1 futures.
use futures::compat::Future01CompatExt;

use std::net::{IpAddr, Ipv4Addr};
use std::net::SocketAddr;

fn main() {
  let localhost_v4 = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

  // Set the address to run our socket on.
  let addr = SocketAddr::from((localhost_v4, 3000));

  // Call our `run_server` function, which returns a future. As with every
  // `async fn`, for `run_server` to do anything, the returned future needs to
  // be run. Additionally, we need to convert the returned future from a
  // futures 0.3 future into a futures 0.1 future.
  let futures_03_future = run_server(addr);
  let futures_01_future = futures_03_future.unit_error().boxed().compat();

  // Finally, we can run the future to completion using the `run` function
  // provided by Hyper.
  run(futures_01_future);
}

pub async fn run_server(addr: SocketAddr) {
  println!("Listening on http://{}", addr);

  // Create a server bound on the provided address
  let serve_future = Server::bind(&addr)
    // Serve requests using our `async serve_fn` function. `serve` takes a
    // closure which returns a type implementing the `Service` trait.
    // `service_fn` returns a value implementing the `Service` trait, and
    // accepts a closure which goes from request to a future of the response.
    // To use our `serve_fn` function with Hyper, we have to box it and put it
    // in a compatability wrapper to go from a futures 0.3 future (the kind
    // returned by `async fn`) to a futures 0.1 future (the kind used by Hyper).
    .serve(|| service_fn(|req| serve_fn(req).boxed().compat()));

  // Wait for the server to complete serving or exit with an error. If an error
  // occurred, print it to stderr.
  if let Err(e) = serve_future.compat().await {
    eprintln!("server error: {}", e);
  }
}

async fn serve_fn(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let mut response = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      *response.body_mut() = Body::from("Try POSTing data to /echo");
    },
    (&Method::POST, "/echo") => {
      *response.body_mut() = req.into_body();
    },
    _ => {
      *response.status_mut() = StatusCode::NOT_FOUND;
    },
  };

  // Always return successfully with a response containing a body with a
  // friendly greeting ;)
  // Ok(Response::new(Body::from("hello, world!")))

  Ok(response)
}
