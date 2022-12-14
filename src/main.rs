use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_files::Files;
use actix_web::{middleware, App, HttpServer, get, Responder, HttpRequest};
use actix_files::NamedFile;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use uuid::Uuid;
use chrono::prelude::*;
use std::env;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers(); 
    log::info!("{} {:?} visiting website - {:?}", txid, peer, requ);
    NamedFile::open_async("./static/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let readi: DateTime<Utc> = Utc::now();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("privkey.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    // WARNING - openssl defaults to TLSv1.0, TLSv1.1, and TLSv1.2 support with TLSv1.3 disabled!
    // work around - compiling outside of the Dockerfile builder and using openssl 1.1.1 can at least disable 1.0 and 1.1...
    //builder.set_ciphersuites("TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256:TLS_AES_128_CCM_8_SHA256:TLS_AES_128_CCM_SHA256").unwrap();
    log::info!("morpho2 initialized at {} >>> morpho2 HTTPS server on port 443 using openssl TLSv1.2 compat mode", readi);
    HttpServer::new(|| {
        App::new()
            .wrap(RedirectHttps::default())
            .wrap(RedirectHttps::with_hsts(StrictTransportSecurity::recommended()))
            .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
            .wrap(middleware::Logger::new("%{txid}e %a -> HTTP %s %r size: %b server-time: %T %{Referer}i %{User-Agent}i"))
            .service(index)
            .service(Files::new("/", "static"))
    })

    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}
