use axum::{body::Body, http::{Request, Response, Error}};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug)]
struct UserId(String);

pub async fn handle(request: Request<Body>) -> Result<Response<Body>, Error> {
    // Access the `UserId` that was set in `on_authorized`. If `handle` gets caslled the
    // request was authorized and `UserId` will be present.
    let user_id = request
        .extensions()
        .get::<UserId>()
        .expect("UserId will be there if request was authorized");

    println!("request from {:?}", user_id);
    main();
    Ok(Response::new(Body::empty()))
}


fn main() {
    let hashed = hash("hunter2", DEFAULT_COST).unwrap();
    let valid = verify("hunter2", &hashed).unwrap();
    println!("{:?}", valid);
}