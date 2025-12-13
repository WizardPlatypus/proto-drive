use sqlx::{Result, Executor, Postgres};
use uuid::Uuid;

pub struct Config {
    pub user_id: Uuid,
    pub sorted: Option<String>,
    pub ascending: bool,
    pub created_at: bool,
    pub edited_at: bool,
    pub owned_by: bool,
    pub edited_by: bool,
    pub filtered: bool,
}

pub async fn init<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO configs (user_id)
        VALUES ($1);
        "#,
        user_id
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn get<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid
) -> Result<Config> {
    sqlx::query_as!(
        Config,
        r#"
        SELECT user_id, sorted, ascending, created_at, edited_at, owned_by, edited_by, filtered
        FROM configs
        WHERE user_id = $1;
        "#,
        user_id
    )
    .fetch_one(e)
    .await
}

    pub async fn update_ascending<'e, E: Executor<'e, Database = Postgres>>(
        e: E,
        user_id: &Uuid,
        value: bool
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE configs
            SET ascending = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(e)
        .await?;
        Ok(())
    }

pub async fn update_created_at<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET created_at = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn update_edited_at<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET edited_at = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn update_owned_by<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET owned_by = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn update_edited_by<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET edited_by = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn update_filtered<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET filtered = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn update_sorted<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    value: Option<String>
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE configs
        SET sorted = $2
        WHERE user_id = $1;
        "#,
        user_id,
        value
    )
    .execute(e)
    .await?;
    Ok(())
}
