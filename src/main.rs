mod options;
mod response;

#[macro_export]
macro_rules! default_bind_address {
    () => ("0.0.0.0:80".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = match options::matches(std::env::args().collect())? {
        None => { return Ok(()) }
        Some(matches) => { matches }
    };

    let verbose = !matches.opt_present("quiet");

    if verbose { eprintln!("initializing..."); }

    let address = if matches.opt_present("bind") {
        matches.opt_get::<String>("bind").unwrap().unwrap()
    } else {
        default_bind_address!()
    };

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route("/*", actix_web::web::get().to(response::response))
            .wrap(actix_web::middleware::Compress::new(
                if matches.opt_present("enable-compression") {
                    actix_web::http::ContentEncoding::Auto
                } else {
                    actix_web::http::ContentEncoding::Identity
                }))
    });
    if verbose { eprintln!("binding to {}...", address); }
    let server = server.bind(address)?;
    if verbose { eprintln!("running..."); }
    let future = server.run();
    if verbose { eprintln!("ready to serve"); }
    future.await?;
    if verbose { eprintln!("exited gracefully"); }
    Ok(())
}