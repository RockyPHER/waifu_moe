use sea_orm::DatabaseConnection;

use crate::repository::anime_repo;
use crate::model::schema_animes::Model as Anime;

pub async fn get_animes_services(
    db: &DatabaseConnection,
) -> Result<Vec<Anime>, sea_orm::DbErr> {
    print!("again");
    anime_repo::get_animes_repo(db).await
}