use crate::model::schema_animes::Model as Anime;
use crate::model::schema_characters::{ActiveModel, Model as Character};
use crate::repository::{anime_repo, character_repo};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn get_animes_services(db: &DatabaseConnection) -> Result<Vec<Anime>, sea_orm::DbErr> {
    anime_repo::get_animes_repo(db).await
}
pub async fn get_anime_services(
    id: i64,
    db: &DatabaseConnection,
) -> Result<std::option::Option<Anime>, sea_orm::DbErr> {
    anime_repo::get_anime_repo(id, db).await
}
pub async fn get_anime_character_service(
    id: i64,
    db: &DatabaseConnection,
) -> Result<Vec<Character>, sea_orm::DbErr> {
    print!("again");
    character_repo::get_anime_character_repo(id, db).await
}
pub async fn patch_character_likes_service(
    id: i64,
    name: String,
    db: &DatabaseConnection,
) -> Result<Character, sea_orm::DbErr> {
    let add_likes: i64 = 1;
    match character_repo::patch_character_likes_repo(id, name.clone(), db).await {
        Ok(Some(character)) => {
            let mut active_character = ActiveModel::from(character);

            // Update the number of likes
            if let Some(current_likes) = active_character.num_likes.unwrap() {
                active_character.num_likes = Set(Some(current_likes + add_likes));
            } else {
                active_character.num_likes = Set(Some(add_likes));
            }

            // Save the updated ActiveModel
            let updated_character = active_character.update(db).await;
            Ok(updated_character.unwrap())
        }
        Ok(None) => Err(sea_orm::DbErr::Custom(format!(
            "Character '{}' not found",
            name
        ))),
        Err(err) => Err(err), // Propagate database error
    }
}
