use sqlx::{Result, Executor, Postgres};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    file_id: &Uuid,
    name: &str,
    path: Option<&str>,
    owner_id: &Uuid
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO files (id, name, path, owned_by)
        VALUES ($1, $2, $3, $4);
        "#,
        file_id,
        name,
        path,
        owner_id
    )
    .execute(e)
    .await?;
    Ok(())
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

pub async fn find<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    like: &str,
    owner_id: &Uuid,
    order_by: Option<&str>,
) -> Result<Vec<File>> {
    let pattern = format!("{like}%");
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE owned_by = $1 AND name LIKE $2 AND deleted_by = NULL AND deleted_at = NULL
        ORDER BY $3
        "#,
        owner_id,
        pattern,
        order_by.unwrap_or("name"),
    )
    .fetch_all(e)
    .await
}

pub async fn find_paged<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    like: &str,
    owner_id: &Uuid,
    order_by: &str,
    limit: i64,
    offset: i64
) -> Result<Vec<File>> {
    let pattern = format!("{like}%");
    sqlx::query_as!(
        File,
        r#"
        SELECT id, name, path, owned_by, edited_by, created_at, edited_at
        FROM files
        WHERE owned_by = $1 AND name LIKE $2 AND deleted_by = NULL AND deleted_at = NULL
        ORDER BY $3
        LIMIT $4
        OFFSET $5
        "#,
        owner_id,
        pattern,
        order_by,
        limit,
        offset
    )
    .fetch_all(e)
    .await
}
