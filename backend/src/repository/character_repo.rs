use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::model::schema_characters::{Column, Entity as Characters, Model as Character};
pub async fn get_characters_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
    Characters::find().all(db).await
}
pub async fn get_anime_character_repo(
    id: i64,
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
    Characters::find()
        .filter(Column::AnimeId.eq(id))
        .all(db)
        .await
}
pub async fn patch_character_likes_repo(
    id: i64,
    name: String,
    db: &DatabaseConnection,
) -> Result<Option<Character>, sea_orm::DbErr> {
    Characters::find()
        .filter(Column::AnimeId.eq(id))
        .filter(Column::Name.eq(name))
        .one(db)
        .await
}
