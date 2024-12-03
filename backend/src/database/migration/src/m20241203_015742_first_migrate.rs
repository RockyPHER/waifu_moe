use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the `Character` table
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Character::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).string().not_null())
                    .col(ColumnDef::new(Character::Bio).string())
                    .col(ColumnDef::new(Character::Age).big_integer())
                    .col(ColumnDef::new(Character::Birthday).date())
                    .col(ColumnDef::new(Character::Height).big_integer())
                    .col(ColumnDef::new(Character::Weight).big_integer())
                    .col(ColumnDef::new(Character::Gender).char_len(1))
                    .col(ColumnDef::new(Character::CardUrl).string())
                    .col(ColumnDef::new(Character::PannelUrl).string())
                    .col(ColumnDef::new(Character::NumLikes).big_integer())
                    .col(ColumnDef::new(Character::NumDisLikes).big_integer())
                    .col(ColumnDef::new(Character::UpdatedAt).date())
                    .col(ColumnDef::new(Character::CreatedAt).date())
                    .to_owned(),
            )
            .await?;

        // Create the `Anime` table
        manager
            .create_table(
                Table::create()
                    .table(Anime::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Anime::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Anime::Name).string().not_null())
                    .col(ColumnDef::new(Anime::CharacterId).big_integer().not_null())
                    .col(ColumnDef::new(Anime::Origin).string())
                    .col(ColumnDef::new(Anime::LogoUrl).string())
                    .col(ColumnDef::new(Anime::UpdatedAt).date())
                    .col(ColumnDef::new(Anime::CreatedAt).date())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Anime::Table, Anime::CharacterId)
                            .to(Character::Table, Character::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the `Anime` table
        manager
            .drop_table(Table::drop().table(Anime::Table).to_owned())
            .await?;

        // Drop the `Character` table
        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Anime {
    Table,
    Id,
    Name,
    CharacterId,
    Origin,
    LogoUrl,
    UpdatedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Character {
    Table,
    Id,
    Name,
    Bio,
    Age,
    Birthday,
    Height,
    Weight,
    Gender,
    CardUrl,
    PannelUrl,
    NumLikes,
    NumDisLikes,
    UpdatedAt,
    CreatedAt,
}
