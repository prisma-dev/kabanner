//! Contains routes for '/user'. Handles CRUD for the `User` model.

use kb_entities::user::{self, Entity as User};
use rocket::{get, http::Status, routes, serde::json::Json, Route};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

use crate::pool::DatabaseHandle;

pub fn routes() -> Vec<Route> {
    routes![all]
}

#[get("/")]
async fn all(db: Connection<'_, DatabaseHandle>) -> Result<Json<Vec<user::Model>>, Status> {
    let conn = db.into_inner();
    let users = User::find().all(conn).await.unwrap();
    Ok(Json(users))
}
