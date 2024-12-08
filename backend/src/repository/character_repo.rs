use crate::model::schema_animes::Column as ColumnAnimes;
use crate::model::schema_characters::{self, Column, Entity as Characters, Model as Character};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
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
    anime: String,
    character: String,
    db: &DatabaseConnection,
) -> Result<Option<Character>, sea_orm::DbErr> {
    Characters::find()
        .join(
            JoinType::InnerJoin,
            schema_characters::Relation::Animes.def(),
        )
        .filter(ColumnAnimes::Name.eq(anime))
        .filter(Column::Name.eq(character))
        .one(db)
        .await
}
pub async fn patch_character_dislikes_repo(
    anime: String,
    character: String,
    db: &DatabaseConnection,
) -> Result<Option<Character>, sea_orm::DbErr> {
    Characters::find()
        .join(
            JoinType::InnerJoin,
            schema_characters::Relation::Animes.def(),
        )
        .filter(ColumnAnimes::Name.eq(anime))
        .filter(Column::Name.eq(character))
        .one(db)
        .await
}
