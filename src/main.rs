use z19_https_server::run;

/* now irrl example:
async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)  // formats to caller return type which impls Responder
}
*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random IP");
    return run(listener)?.await;
}
