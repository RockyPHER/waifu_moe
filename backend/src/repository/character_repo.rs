use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::model::{schema_characters::{Entity as Characters, Model as Character, Column}};
pub async fn get_characters_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
     Characters::find().all(db).await
}
pub async fn get_anime_character_repo(
    id: i64,
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
    Characters::find().filter(Column::AnimeId.eq(id)).all(db).await
}