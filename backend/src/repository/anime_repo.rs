use sea_orm::{DatabaseConnection, EntityTrait};
use crate::model::schema_animes::{Entity as Animes, Model as Anime};
pub async fn get_animes_repo(
    db: &DatabaseConnection,
) -> Result<Vec<Anime>, sea_orm::DbErr> {
    print!("friend of a friend");
    Animes::find().all(db).await
}
