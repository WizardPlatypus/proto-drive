use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: Option<String>, // is null for folders
    pub owned_by: Uuid,
    pub edited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

pub async fn create<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    name: &str,
    path: Option<&str>,
    owner_id: &Uuid,
) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO files (name, path, owned_by)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        name,
        path,
        owner_id
    )
    .fetch_one(e)
    .await?;
    Ok(rec.id)
}

pub async fn delete<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    file_id: &Uuid,
    user_id: &Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE files
        SET deleted_by = $1,
            deleted_at = now()
        WHERE id = $2;
        "#,
        user_id,
        file_id,
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn edit<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    file_id: &Uuid,
    user_id: &Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE files
        SET edited_by = $1,
            edited_at = now()
        WHERE id = $2;
        "#,
        user_id,
        file_id,
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn r#move<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    file_id: &Uuid,
    path: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE files
        SET path = $1
        WHERE id = $2;
        "#,
        path,
        file_id,
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn rename<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    file_id: &Uuid,
    name: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE files
        SET name = $1
        WHERE id = $2;
        "#,
        name,
        file_id,
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn find_by_id<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    file_id: &Uuid,
) -> Result<Option<File>> {
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE id = $1;
        "#,
        file_id
    )
    .fetch_optional(e)
    .await
}

pub async fn regex<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    query: &str,
    owner_id: &Uuid,
    order_by: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<File>> {
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE owned_by = $1 AND name ~ $2 AND deleted_by = NULL AND deleted_at = NULL
        ORDER BY $3
        LIMIT $4
        OFFSET $5;
        "#,
        owner_id,
        query,
        order_by,
        limit,
        offset
    )
    .fetch_all(e)
    .await
}

pub async fn folder<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    owner_id: &Uuid,
    name: &str,
) -> Result<Vec<File>> {
    let pattern = format!("'{}{}'", name.replace("/", r"\/"), r"\/(\w|\.)+");
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE owned_by = $1 AND name ~ $2 AND deleted_by = NULL AND deleted_at = NULL;
        "#,
        owner_id,
        &pattern
    )
    .fetch_all(e)
    .await
}

pub async fn root<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    owner_id: &Uuid,
) -> Result<Vec<File>> {
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE owned_by = $1 AND name ~ '(\w|\.)+' AND deleted_by = NULL AND deleted_at = NULL;
        "#,
        owner_id
    )
    .fetch_all(e)
    .await
}
