use axum::{
    extract::Path,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/*path", get(static_pages));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn static_pages(path: Path<String>) -> (StatusCode, Html<String>) {
    let path = std::path::PathBuf::from("./src/static").join(path.0);
    let data = std::fs::read_to_string(path);

    match data {
        Ok(data) => (StatusCode::OK, Html(data)),
        Err(x) => match x.kind() {
            std::io::ErrorKind::NotFound => (
                StatusCode::NOT_FOUND,
                Html("<h1>Not Found</h1>".to_string()),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("<h1>INTERNAL_SERVER_ERROR</h1>".to_string()),
            ),
        },
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
