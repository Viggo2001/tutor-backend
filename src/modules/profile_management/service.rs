use crate::modules::profile_management::repository;
use crate::modules::profile_management::model::User;

pub async fn fetch_users() -> Vec<User> {
    repository::get_all_users().await
}

