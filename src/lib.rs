use {
    actix_web::{
        web, App,
        HttpRequest, HttpServer,
        Responder, HttpResponse,
        dev::Server,
    },
    std::{
        net::TcpListener,  
    },
};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run(); 

    return Ok(server); // no longer using await, return server obj.
}

// Using HttpResponse as return value moving forward.
async fn health_check(_req: HttpRequest) -> impl Responder{
    return HttpResponse::Ok().finish(); // can drop finish() since struct impls Responder
}

async fn subscribe() -> HttpResponse{
    return HttpResponse::Ok().finish();
}
