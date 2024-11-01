use actix::Addr;
use actix_web::{
    get, post, web::{Data, Json, Path}, HttpResponse, Responder
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser, FetchUserArticles},
    AppState,
    DbActor,
};

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users")
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles."),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    format!("POST /users/{id}/articles")
}