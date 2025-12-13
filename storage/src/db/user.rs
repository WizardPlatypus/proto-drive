use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub login: String,
    pub phc: String,
}

pub async fn create<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    login: &str,
    phc: &str,
) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (login, phc)
        VALUES ($1, $2)
        RETURNING id;
        "#,
        login,
        phc
    )
    .fetch_one(e)
    .await?;
    Ok(rec.id)
}

pub async fn delete<'e, E: Executor<'e, Database = Postgres>>(e: E, id: &Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1;
        "#,
        id
    )
    .execute(e)
    .await?;
    Ok(())
}

pub async fn find_by_id<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    id: &Uuid,
) -> Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, login, phc
        FROM users
        WHERE id = $1;
        "#,
        id
    )
    .fetch_optional(e)
    .await
}

pub async fn find_by_login<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    login: &str,
) -> Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, login, phc
        FROM users
        WHERE login = $1;
        "#,
        login
    )
    .fetch_optional(e)
    .await
}

pub async fn login_exists<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    login: &str,
) -> Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE login = $1
        );
        "#,
        login
    )
    .fetch_one(e)
    .await?;
    Ok(exists.unwrap_or(false))
}

pub async fn update_password<'e, E: Executor<'e, Database = Postgres>>(
    e: E,
    user_id: &Uuid,
    new_phc: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE users
        SET phc = $1
        WHERE id = $2;
        "#,
        new_phc,
        user_id
    )
    .execute(e)
    .await?;
    Ok(())
}
