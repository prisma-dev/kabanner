//! Contains routes for '/users'. Handles CRUD for the `User` model.

use kb_entities::user::{self, Entity as User};
use rocket::{get, http::Status, routes, serde::json::Json, Data, Route};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

use crate::pool::DatabaseHandle;

/// Returns a [`Vec<Route>`] with all the routes related to [`User`]'s.
pub fn routes() -> Vec<Route> {
    routes![all, view]
}

#[get("/")]
async fn all(db: Connection<'_, DatabaseHandle>) -> Result<Json<Vec<user::Model>>, Status> {
    let conn = db.into_inner();
    User::find()
        .all(conn)
        .await
        .map_err(|_| Status::InternalServerError)
        .map(|users| Json(users))
}

#[get("/<id>")]
async fn view(db: Connection<'_, DatabaseHandle>, id: i32) -> Result<Json<user::Model>, Status> {
    let conn = db.into_inner();
    match User::find_by_id(id)
        .one(conn)
        .await
        .map_err(|_| Status::InternalServerError)?
    {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}
