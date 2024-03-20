use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind_openssl(("0.0.0.0", 8080))?
    .run()
    .await
}


pub fn tls_acceptor() -> native_tls::TlsAcceptor {
    let mut file = std::fs::File::open("cert+key.p12")
        .map_err(|e| {
            println!("failed to open .p12");
            println!("try running: openssl pkcs12 -export -passout pass:'' -out cert+key.p12 -inkey key.pem -in cert.pem");
            e
        }).expect("opening .p12");
    let mut der = vec![];
    file.read_to_end(&mut der).unwrap();
    let cert = native_tls::Identity::from_pkcs12(&der, "").expect("failed to read .p12");
    let tls_cx = native_tls::TlsAcceptor::builder(cert).build().unwrap();
    native_tls::TlsAcceptor::from(tls_cx)
}
