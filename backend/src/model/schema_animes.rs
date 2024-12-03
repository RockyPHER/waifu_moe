use sea_orm::entity::prelude::*;
use serde::Serialize;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "animes")]

pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub origin: Option<String>,
    pub logo_url: Option<String>,
    pub updated_at: Option<Date>,
    pub created_at: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::schema_characters::Entity")]
    Character,
}
impl Related<super::schema_characters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
