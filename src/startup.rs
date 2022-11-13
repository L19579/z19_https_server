use {
  crate::*, 
};

pub fn run(listener: TcpListener, db_conn_pool: PgPool) 
-> Result::<Server, std::io::Error>{
    //Essentially an Arc<T> since PgConnection's TCP resource isn't cloneable.
    let db_conn_pool = web::Data::new(db_conn_pool); 
    let server = HttpServer::new(move ||{
        App::new()
            .wrap(middleware::Logger::default()) //emits log for incoming reqs.
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_conn_pool.clone()) // <-- app state / persistence
    })
    .listen(listener)?
    .run();

    return Ok(server);
}
