use crate::*;

pub fn run(listener: TcpListener) -> Result::<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    return Ok(server);
}