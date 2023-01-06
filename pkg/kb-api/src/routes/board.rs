//! Contains routes for '/boards'. Handles CRUD for the `Board` model.

use chrono::Utc;
use kb_entities::{
    board,
    prelude::{Board, User},
};
use rocket::{
    get,
    http::Status,
    post, routes,
    serde::{json::Json, Deserialize},
    Route,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm_rocket::Connection;

use crate::pool::DatabaseHandle;

/// Returns a [`Vec<Route>`] with all the routes related to [`Board`]'s.
pub fn routes() -> Vec<Route> {
    routes![new, all]
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct NewBoardDto<'n> {
    name: &'n str,
    owner_id: i32,
}

#[post("/", data = "<dto>")]
async fn new(
    db: Connection<'_, DatabaseHandle>,
    dto: Json<NewBoardDto<'_>>,
) -> Result<Json<board::Model>, Status> {
    let conn = db.into_inner();

    let user = User::find_by_id(dto.owner_id)
        .one(conn)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if user.is_none() {
        return Err(Status::BadRequest);
    }

    let board = board::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(dto.name.to_owned()),
        owner_id: ActiveValue::Set(dto.owner_id),
        created_at: ActiveValue::Set(Utc::now()),
        updated_at: ActiveValue::Set(Utc::now()),
    };

    board
        .insert(conn)
        .await
        .map(|board| Json(board))
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
async fn all(db: Connection<'_, DatabaseHandle>) -> Result<Json<Vec<board::Model>>, Status> {
    let conn = db.into_inner();
    Board::find()
        .all(conn)
        .await
        .map(|boards| Json(boards))
        .map_err(|_| Status::InternalServerError)
}
