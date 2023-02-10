use clap::Parser;
use server::router::get_router;
use server::structs::opt_struct::Opt;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use server::database::{connect_to_db};

#[tokio::main]

async fn main() {
    // println!("{}",_db);
    let opt = Opt::parse();
    println!("{:?}", opt);
    println!("after _db initialized");
    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();
    let router = get_router();
    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    log::info!("listening on http://{}", sock_addr);
    axum::Server::bind(&sock_addr)
        .serve(router.await.into_make_service())
        .await
        .expect("Unable to start server");
    
    // let _client = connect_to_db().await.expect("error connecting to db.");
    // dbg!(_client);
}
