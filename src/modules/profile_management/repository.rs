use crate::db;
use crate::modules::profile_management::model::User;

pub async fn get_all_users() -> Vec<User> {
    let pool = db::connect_db().await;
    let rows = sqlx::query_as!(
        User,
        r#"
        SELECT 
            user_id,
            first_name,
            last_name,
            email,
            password,
            role as "role: _",
            created_at
        FROM profile_management.user
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch users");

    rows
}
