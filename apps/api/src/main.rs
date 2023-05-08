use axum::{routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
}

async fn index() -> Json<Message> {
    let data = Message {
        message: "Hello, World!".to_string(),
    };
    Json(data)
}

async fn bye() -> String {
    "Bye, World!".to_string()
}

async fn create_item() -> String {
    "Created".to_string()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/bye", get(bye))
        .route("/items", post(create_item));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
