pub mod prelude {
    pub use super::anime::Entity as Anime;
}

pub mod anime {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "anime")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub character_id:  i32,
        pub origin: String,
        pub logo_url: String,
        pub updated_at: NaiveDateTime,
        pub created_at: Na
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "anime::Entity",
            from = "Column::id",
            to = "anime::Column::character_id"
        )]
        Character,
    }

    impl ActiveModelBehavior for ActiveModel {}
}
