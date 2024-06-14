use auto_rust::llm_tool;
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, IntoResponse,
    Route, Server,
};

#[llm_tool]
#[handler]
/// The response will be an html layout with the name and the number of letters in the name.
/// Ensure to return a content type of text/html.
fn hello(Path(name): Path<String>) -> impl IntoResponse {
    todo!()
}

#[llm_tool]
#[handler]
/// The response will be an html layout with the number of times the page has been visited.
/// The count will be stored in a file.
/// The count will be incremented each time the page is visited.
/// The count will be displayed on the page.
fn count() -> impl IntoResponse {
    todo!()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/count", get(count))
        .with(Tracing);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
