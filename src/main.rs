use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};
use http::{HeaderValue, header};
use reqwest::Client;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

const LISTEN_ADDR: &str = "0.0.0.0:8808";
// const LISTEN_ADDR: &str = "127.0.0.1:8808"; // for testing

// Yew content directory
const STATIC_DIR: &str = "./dist";
// Zenn RSS feed URL
const URL: &str = "https://zenn.dev/amenaruya/feed?all=1";

#[tokio::main]
async fn main() {
    // define content directory
    let serve_dir: ServeDir = ServeDir::new(STATIC_DIR);
    // define app
    let router: Router = Router::new()
        .route("/", get(root))
        .route("/feed", get(fetch))
        .nest_service("/dist", serve_dir)
        .layer(CorsLayer::permissive());
    // define listener
    let listener: TcpListener = TcpListener::bind(LISTEN_ADDR).await.unwrap();

    println!("Listening on port 8808");

    axum::serve(listener, router.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn root() -> Redirect {
    Redirect::permanent("/dist")
}

async fn fetch() -> impl IntoResponse {
    // client
    let client: Client = Client::new();
    // request
    let content: reqwest::Request = client.get(URL).build().unwrap();
    // let res: Result<Response, Error> =  client.execute(content).await;

    // body
    let body: axum::body::Bytes = client
        .execute(content)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let head: [(http::HeaderName, HeaderValue); 1] = [(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/xml"),
    )];
    let res: http::Response<axum::body::Body> = (head, body).into_response();
    res
}
