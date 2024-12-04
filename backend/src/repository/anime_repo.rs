use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::model::{schema_animes::{Entity as Animes, Model as Anime}, schema_characters::Model};
pub async fn get_animes_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Anime>, sea_orm::DbErr> {
    Animes::find().all(db).await
}
pub async fn get_anime_repo(
    id: i64,
    db: &DatabaseConnection,
) -> Result<&Anime, sea_orm::DbErr> {
    Animes::find().filter()
}
