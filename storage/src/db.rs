use uuid::Uuid;
use sqlx::{PgPool, Result, Executor};
use chrono::{DateTime, Utc};

pub struct User {
    pub id: Uuid,
    pub login: String,
    pub phc: String,
}

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: Option<String>, // is null for folders
    pub owned_by: Uuid,
    pub edited_by: Option<Uuid>,
    // pub deleted_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    // pub deleted_at: Option<DateTime<Utc>>
}

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

// #####
// USERS
// #####

pub async fn create_user(
    pool: &PgPool,
    id: &Uuid,
    login: &str,
    phc: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, login, phc)
        VALUES ($1, $2, $3);
        "#,
        id,
        login,
        phc
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user(
    pool: &PgPool,
    id: &Uuid
) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1;
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user_by_id(
    pool: &PgPool,
    id: &Uuid
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
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_login(
    pool: &PgPool,
    login: &str
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
    .fetch_optional(pool)
    .await
}

pub async fn login_exists(
    pool: &PgPool,
    login: &str
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
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap_or(false))
}

pub async fn update_password(
    pool: &PgPool,
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
    .execute(pool)
    .await?;

    Ok(())
}


// #####
// FILES
// #####

pub async fn create_file(
    pool: &PgPool,
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
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_file(
    pool: &PgPool,
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
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn edit_file(
    pool: &PgPool,
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
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_file_by_id(
    pool: &PgPool,
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
    .fetch_optional(pool)
    .await
}

pub async fn find_all_files(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

pub async fn find_paged_files(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await
}

// #######
// CONFIGS
// #######

pub async fn init_config(
    pool: &PgPool,
    user_id: &Uuid
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO configs (user_id)
        VALUES ($1);
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_config(
    pool: &PgPool,
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
    .fetch_one(pool)
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn update_config(
    pool: &PgPool,
    user_id: &Uuid,
    sorted: Option<String>,
    ascending: Option<bool>,
    created_at: Option<bool>,
    edited_at: Option<bool>,
    owned_by: Option<bool>,
    edited_by: Option<bool>,
    filtered: Option<bool>,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    if let Some(value) = sorted {
        if value.is_empty() {
            sqlx::query!(
                r#"
                UPDATE configs
                SET sorted = NULL
                WHERE user_id = $1;
                "#,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        } else {
            sqlx::query!(
                r#"
                UPDATE configs
                SET sorted = $2
                WHERE user_id = $1;
                "#,
                user_id,
                value
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    if let Some(value) = ascending {
        sqlx::query!(
            r#"
            UPDATE configs
            SET ascending = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(value) = created_at {
        sqlx::query!(
            r#"
            UPDATE configs
            SET created_at = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(value) = edited_at {
        sqlx::query!(
            r#"
            UPDATE configs
            SET edited_at = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(value) = owned_by {
        sqlx::query!(
            r#"
            UPDATE configs
            SET owned_by = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(value) = edited_by {
        sqlx::query!(
            r#"
            UPDATE configs
            SET edited_by = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(value) = filtered {
        sqlx::query!(
            r#"
            UPDATE configs
            SET edited_by = $2
            WHERE user_id = $1;
            "#,
            user_id,
            value
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

// Sadly, does not work
/*
pub async fn update_config_column(
    pool: &PgPool,
    user_id: &Uuid,
    column: &str,
    value: bool
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE config
        SET $1 = $2
        WHERE user_id = $3;
        "#,
        column,
        value,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
// */