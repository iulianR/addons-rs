use chrono::{Utc};

use sqlx::{sqlite::SqlitePool};
use std::env;

use crate::model::{AddonRecord, CategoryRecord, GameRecord};

pub async fn setup_db() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect(env!("DATABASE_URL")).await?;

    return Ok(pool);
}

pub async fn all_crates(pool: &SqlitePool) -> anyhow::Result<Vec<AddonRecord>> {
    Ok(sqlx::query_as!(
        AddonRecord,
        r#"
    SELECT id, name
    FROM addons
    ORDER BY id
            "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn all_games(pool: &SqlitePool) -> anyhow::Result<Vec<GameRecord>> {
    Ok(sqlx::query_as!(
        GameRecord,
        r#"
    SELECT id, name
    FROM games
    ORDER BY id
            "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn all_categories(pool: &SqlitePool) -> anyhow::Result<Vec<CategoryRecord>> {
    Ok(sqlx::query_as!(
        CategoryRecord,
        r#"
    SELECT id, name, game
    FROM categories
    ORDER BY id
            "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn categories_for_game(
    game_id: i64,
    pool: &SqlitePool,
) -> anyhow::Result<Vec<CategoryRecord>> {
    Ok(sqlx::query_as!(
        CategoryRecord,
        r#"
    SELECT id, name, game
    FROM categories
    WHERE game = (?1)
    ORDER BY id
            "#,
        game_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn insert_addon(addon: &str, pool: &SqlitePool) -> anyhow::Result<()> {
    let _now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO addons (name) values (?1)
    "#,
        addon,
    )
    .execute(pool)
    .await?;

    Ok(())
}
