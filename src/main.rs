use axum::{
    Router,
    extract::Path,
    routing::{get, post},
};
use axum_cbor::{Cbor, CborRejection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
}

async fn get_user(Path(user_id): Path<String>) -> Result<Cbor<User>, CborRejection> {
    // Simulate fetching user from database
    let user = User {
        id: user_id,
        username: "example_user".to_string(),
    };
    Ok(Cbor(user))
}

async fn create_user(Cbor(new_user): Cbor<User>) -> Result<Cbor<User>, CborRejection> {
    // Simulate creating user in database
    Ok(Cbor(new_user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user));

    let server_url = "0.0.0.0:3550";
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    println!("Server running on {}", server_url);
    axum::serve(listener, app).await.unwrap();
}
