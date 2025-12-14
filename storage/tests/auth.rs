use sqlx::PgPool;

#[sqlx::test]
async fn register_success(pool: PgPool) {
    let user_id = storage::auth::register_user(&pool, "algernon", "flowers")
        .await
        .unwrap();
    assert!(!user_id.is_nil())
}

#[sqlx::test]
async fn login_success(pool: PgPool) {
    let login = "algernon";
    let password = "flowers";
    let user_id = storage::auth::register_user(&pool, login, password)
        .await
        .unwrap();
    let logged_in = storage::auth::login_user(&pool, login, password)
        .await
        .unwrap();
    assert_eq!(user_id, logged_in);
}

#[sqlx::test]
async fn login_fail(pool: PgPool) {
    let login = "algernon";
    let password = "flowers";
    let _user_id = storage::auth::register_user(&pool, login, password)
        .await
        .unwrap();
    let logged_in = storage::auth::login_user(&pool, login, "other").await;
    assert!(logged_in.is_err());
}

#[sqlx::test]
async fn verify_test_db(pool: PgPool) {
    let row = sqlx::query!("SELECT current_database()")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("connected to {}", row.current_database.unwrap());
}
