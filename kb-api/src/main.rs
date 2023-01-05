mod pool;
mod routes;

use anyhow::Result;
use sea_orm_rocket::Database;

use pool::DatabaseHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let _rocket = rocket::build()
        .attach(DatabaseHandle::init())
        .mount("/users", routes::user::routes())
        .launch()
        .await?;
    Ok(())
}
