use axum::routing::{get,post,put,delete};
use axum::{Router, Json};
use serde::Deserialize;

#[tokio::main]
async fn  main() {
    let app=Router::new()
            .route("/",get(home))
            .route("/greet",get(greet))
            .route("/login", post(login))
            .route("/update", put(update))
            .route("/delete/{id}",delete(delete_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
                    .await
                    .unwrap();
        println!("Server is Running at http://localhos:8080");

        axum::serve(listener, app)
            .await
            .unwrap();
}

async fn home() -> String{
    String::from("API is Running...!")
}

async fn greet() -> String{
    String::from("Hello user.. welcome")
}

#[derive(Deserialize)]
struct LoginRequest{
    username: String,
    password: String
}

async fn login(Json(data):Json<LoginRequest>) -> String{
    if data.username == "admin" && data.password == "123456"{
        "Login Successfull!".to_string()
    }
    else{
        "Invalid username password!".to_string()
    }
}

#[derive(Deserialize)]
struct UpdateRequest{
    id:i32,
    name:String
}

async fn update(Json(data):Json<UpdateRequest>) -> String{
    format!("Name is updated for Id:{}", data.id)
}

async fn delete_user(
    axum::extract::Path(id):axum::extract::Path<i32>
) -> String{
    format!("Record Deleted With ID:{}",id)
}