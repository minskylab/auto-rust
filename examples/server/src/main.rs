use auto_rust::llm_tool;
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server,
};

#[llm_tool]
#[handler]
/// This function returns a hello string with the name and the number of letters in the name.
fn hello(Path(name): Path<String>) -> String {
    todo!()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let app = Route::new().at("/hello/:name", get(hello)).with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
