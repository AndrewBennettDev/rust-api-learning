use actix_web::web::Path;
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    let likes = Likes { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}likes")]
pub async fn plus_one(path: Path<(String,)>) -> HttpResponse {
    let like = Like::new();

    HttpResponse::created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent()
        .content(APPLICATION_JSON)
        .await
        .unwrap()
}
