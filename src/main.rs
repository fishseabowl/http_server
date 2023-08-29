use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use clap::{App, Arg};
use hyper::server::conn::AddrStream;

fn parse_cli_arguments() -> (String) {
    let matches = App::new("http_server")
        .version("0.1.0")
        .author("domichain")
        .about("Create an HTTP server for testing.")
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("addr")
                .value_name("Server Addreee")
                .required(true)
                .help("Sets server address"),
        )
        .get_matches();

        matches.value_of("addr").unwrap().to_string()

        
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Print the details of the request
    println!("Received request: {:?}", req);
    let body = req.into_body();
    println!("body {:?}", body);
    let full_body = hyper::body::to_bytes(body).await?;

    // Convert the body bytes to a string
    let body_str = String::from_utf8_lossy(&full_body).to_string();

    println!("Body {:?}", body_str);

    // Create a response
    let response = Response::new(Body::from("Hello, World!"));

    // Print the details of the response
    println!("Sending response: {:?}", response);

    Ok(response)
}


#[tokio::main]
async fn main() {
    // Define the address on which the server will listen
    let addr = parse_cli_arguments()

    // Create a new `Service` to handle incoming requests
    let make_svc = make_service_fn(|_conn: &AddrStream| {
        // let remote_addr = _conn.remote_addr();
        // println!("Remote Address: {:?}", remote_addr);
        async {
            // Return a new instance of the service to handle each connection
            Ok::<_, hyper::Error>(service_fn(handle_request))
        }
    });

    /* let make_svc = make_service_fn(|conn: &AddrStream| {
        async { Ok::<_, hyper::Error>(service_fn(move |req| handle_request(conn.clone(), req))) }
    }); */

    // Create a new HTTP server and bind it to the specified address
    println!("Http Server listening on http://{:?}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server and await for it to finish
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
