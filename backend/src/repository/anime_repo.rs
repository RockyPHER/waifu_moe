use sea_orm::{DatabaseConnection, EntityTrait};
use crate::model::schema_animes::{Entity as Animes, Model as Anime};
pub async fn get_animes_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Anime>, sea_orm::DbErr> {
    Animes::find().all(db).await
}
pub async fn get_anime_repo(
    id: i64,
    db: &DatabaseConnection,
) -> Result<std::option::Option<Anime>, sea_orm::DbErr> {
    Animes::find_by_id(id).one(db).await
}

