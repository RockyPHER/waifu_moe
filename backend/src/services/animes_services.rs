use sea_orm::DatabaseConnection;

use crate::repository::{anime_repo, character_repo};
use crate::model::schema_animes::Model as Anime;
use crate::model::schema_characters::Model as Characters;

pub async fn get_animes_services(
    db: &DatabaseConnection,
) -> Result<Vec<Anime>, sea_orm::DbErr> {
    
    anime_repo::get_animes_repo(db).await
}
pub async fn get_anime_services(id: i64,
    db: &DatabaseConnection,
) -> Result<std::option::Option<Anime>, sea_orm::DbErr> {
    anime_repo::get_anime_repo(id,db).await
}
pub async fn get_anime_character_service(
    id: i64,
    db: &DatabaseConnection,
) -> Result<Vec<Characters>, sea_orm::DbErr> {
    print!("again");
    character_repo::get_anime_character_repo(id,db).await
}
