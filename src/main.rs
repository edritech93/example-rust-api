use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("Server run on http://localhost:3000");
    // axum::serve(listener, app).await.unwrap();
}
