// use crate::modules::profile_management::repository;
use crate::modules::profile_management::user::repository;
use crate::modules::profile_management::user::model::User;

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    SUCCESS, FAILED
}

pub async fn fetch_users() -> Vec<User> {
    repository::get_all_users().await
}

pub async fn add_user(user: User) -> Status {
    let result = repository::add_user(user).await;

    match result {
        Ok(_res) => Status::SUCCESS,
        Err(_err) => Status::FAILED
    }
}
