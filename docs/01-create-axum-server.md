# Axum으로 Rust 서버 시작

작업 날짜: 2023-05-09

## Rust 프로젝트 생성

```
cargo init
```

그리고 Cargo.toml에 라이브러리 추가

```
[package]
name = "api"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.6.18"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.96"
```

tokio 는 비동기처리할때 필요함. serde 관련은 JSON 처리 할 때 필요.

안녕 세계! 해보자.

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:8000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

근데 3000 포트 말고 8000을 쓰자. 3000은 리액트 개발 서버에서 쓸테니까.

```
cargo run
```

하면 http://localhost:8000 에 서버 뜸 오 ㅋㅋㅋ

## 프로젝트 watch 하여 자동 재시작

파일 바뀔 때 자동으로 서버가 재시작했으면 좋겠다.

[cargo-watch](https://crates.io/crates/cargo-watch) 라는 도구를 쓰면 됨

```
cargo install cargo-watch
```

설치되고 나면,

```
cargo watch -w ./src -s 'cargo run'
```

이제 코드 수정해보자. 재시작 되나?

개꿀 ~! 잘됨.

## JSON 반환

지금은 그냥 Hello World 주고 있는데 JSON을 주고 싶어.

```
{ "message": "it works!" }
```

이런 메시지를 주고 싶음.

그러면, 코드를 이렇게 작성하면 됨

```rust
use axum::{routing::get, Json, Router};
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

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

```

## route 더 만들어보기

추가 라우트들은 이렇게 만들어보겠음.

- GET /bye
- POST /items

```rust
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
```

잘 되는군..

다음 개발땐 라우트에서 파라미터 받아서 처리도 해보고, route들을 모듈로 분리 할 수 있을지도 알아보자.
