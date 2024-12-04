use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "characters")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub anime_id: i64,
    pub name: String,
    pub bio: Option<String>,
    pub age: Option<i64>,
    pub birthday: Option<Date>,
    pub height: Option<i64>,
    pub weight: Option<i64>,
    pub gender: Option<String>,
    pub card_url: Option<String>,
    pub panel_url: Option<String>,
    pub num_likes: Option<i64>,
    pub num_dislikes: Option<i64>,
    pub updated_at: Option<Date>,
    pub created_at: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::schema_animes::Entity",
        from = "Column::AnimeId",
        to = "super::schema_animes::Column::Id"
    )]
    Anime,
}

impl Related<super::schema_animes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
