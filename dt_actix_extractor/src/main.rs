use actix_web::{get, post, web, App, HttpRequest, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: i32,
    username: String,
}

#[get("/users/v1/{user_id}/{username}")]
async fn index_one(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Elcome {}, user_id {}",
        info.username, info.user_id
    ))
}

#[get("/users/v2/{user_id}/{username}")]
async fn index_two(path: web::Path<(u32, String)>) -> Result<String> {
    let (username, user_id) = path.into_inner();
    Ok(format!("Elcome {}, user_id {}", username, user_id))
}

#[get("/users/v3/{user_id}/{username}")]
async fn index_three(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
    Ok(format!("Elcome {}, user_id {}", username, user_id))
}

#[derive(Deserialize)]
struct InfoQuery {
    username: String,
}

#[get("/query")]
async fn index_query(query: web::Query<InfoQuery>) -> Result<String> {
    Ok(format!("Elcome {}", query.username))
}
#[derive(Deserialize)]
struct InfoJson {
    username: String,
}

#[post("/json")]
async fn index_json(json: web::Json<InfoJson>) -> Result<String> {
    Ok(format!("Elcome {}", json.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index_one)
            .service(index_two)
            .service(index_three)
            .service(index_query)
            .service(index_json)
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
}
