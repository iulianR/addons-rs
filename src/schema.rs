use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use sqlx::SqlitePool;

use crate::{
    db,
    error::ApiError,
    model::{AddonRecord, CategoryRecord, GameRecord},
};

pub type AddonsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub type ApiResult<T> = Result<T, ApiError>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn addons(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Addon>> {
        let db = ctx.data_unchecked::<SqlitePool>();

        Ok(db::all_crates(db)
            .await?
            .into_iter()
            .map(Addon::from)
            .collect())
    }

    async fn games(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Game>> {
        let db = ctx.data_unchecked::<SqlitePool>();

        Ok(db::all_games(db)
            .await?
            .into_iter()
            .map(Game::from)
            .collect())
    }

    async fn categories(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Category>> {
        let db = ctx.data_unchecked::<SqlitePool>();

        Ok(db::all_categories(db)
            .await?
            .into_iter()
            .map(Category::from)
            .collect())
    }
}

#[derive(Debug, SimpleObject)]
pub struct Addon {
    pub id: i64,
    pub name: String,
}

impl From<AddonRecord> for Addon {
    fn from(record: AddonRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub name: String,
}

#[Object]
impl Game {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn categories_for_game(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Category>> {
        let db = ctx.data_unchecked::<SqlitePool>();

        Ok(db::categories_for_game(self.id, db)
            .await?
            .into_iter()
            .map(Category::from)
            .collect())
    }
}

impl From<GameRecord> for Game {
    fn from(record: GameRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

impl From<CategoryRecord> for Category {
    fn from(record: CategoryRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
        }
    }
}
