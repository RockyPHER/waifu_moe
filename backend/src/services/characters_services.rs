use sea_orm::DatabaseConnection;

use crate::repository::character_repo;
use crate::model::schema_characters::Model as Characters;

pub async fn get_characters_service(
    db: &DatabaseConnection,
) -> Result<Vec<Characters>, sea_orm::DbErr> {
    print!("again");
    character_repo::get_characters_repo(db).await
}
