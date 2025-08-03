use sqlx::postgres::PgQueryResult;
use sqlx::Error;

use crate::db;
use crate::modules::profile_management::user::model::User;

use super::model::UserRole;
pub async fn get_all_users() -> Vec<User> {
    let pool = db::connect_db().await;

    // Fetch the rows as tuples
    let rows = sqlx::query!(
        r#"
        SELECT 
            user_id,
            first_name,
            last_name,
            email,
            password,
            role::text AS role_str,  -- Cast enum to text
            created_at
        FROM profile_management.user
        "#
    )
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(rows) => {
            rows.into_iter()
                .filter_map(|row| {
                    let role = match row.role_str.as_deref() {
                        Some("admin") => Some(UserRole::Admin),
                        Some("student") => Some(UserRole::Student),
                        Some("tutor") => Some(UserRole::Tutor),
                        _ => {
                            println!("Unknown role: {:?}", row.role_str);
                            None
                        }
                    };

                    // If role was mapped correctly, build User
                    role.map(|r| User {
                        user_id: row.user_id,
                        first_name: row.first_name,
                        last_name: row.last_name,
                        email: row.email,
                        password: row.password,
                        role: r,
                        created_at: row.created_at,
                    })
                })
                .collect()
        }
        Err(err) => {
            println!("Failed to fetch users: {}", err);
            Vec::new()
        }
    }
}

pub async fn get_user_by_username_and_password(email: &str, password: &str) -> Option<User> {
    let pool = db::connect_db().await;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM profile_management.user WHERE email = $1 AND password = $2"
    )
    .bind(email)
    .bind(password)
    .fetch_optional(&pool)
    .await
    .unwrap_or_else(|error| { error });

    return user;
}

pub async fn add_user(user: User) -> Result<PgQueryResult, Error> {
    let pool = db::connect_db().await;

    let result = sqlx::query( 
        r#"
        INSERT INTO profile_management.user
            (user_id, first_name, last_name, email, password, role, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW());
        "#
    )
        .bind(user.user_id)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(user.password)
        .bind(user.role)
    .execute(&pool)
    .await;

    match result {
        Ok(res) => {
            Ok(res)
        },
        Err(err) => {
            println!("Error occurred: {}", err);
            Err(err)
        }
    }
}
