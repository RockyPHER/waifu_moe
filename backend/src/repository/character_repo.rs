use sea_orm::{DatabaseConnection, EntityTrait};

use crate::model::schema_characters::{Model as Character, Entity as Characters};
pub async fn get_characters_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
     Characters::find().all(db).await
}