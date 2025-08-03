use crate::{db, modules::profile_management::tutor::model::Tutor};
use bigdecimal::BigDecimal;
use sqlx::{Result};


impl Tutor {

    pub async fn find_by_id(tutor_id: &str) -> Result<Option<Tutor>> {
        let pool = db::connect_db().await;

        let tutor = sqlx::query_as::<_, Tutor>(
            "SELECT * FROM profile_management.tutor WHERE tutor_id = $1"
        )
        .bind(tutor_id)
        .fetch_optional(&pool)
        .await?;

        Ok(tutor)
    }

    pub async fn create(new_tutor: Tutor) -> Result<Tutor> {
        let pool = db::connect_db().await;

        let tutor = sqlx::query_as::<_, Tutor>(
            r#"
            INSERT INTO profile_management.tutor (
                tutor_id, user_id, created_at, updated_at, is_available,
                city, country, year, bio, average
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(&new_tutor.tutor_id)
        .bind(&new_tutor.user_id)
        .bind(new_tutor.created_at)
        .bind(new_tutor.updated_at)
        .bind(new_tutor.is_available)
        .bind(&new_tutor.city)
        .bind(&new_tutor.country)
        .bind(new_tutor.year)
        .bind(&new_tutor.bio)
        .bind(new_tutor.average)
        .fetch_one(&pool)
        .await?;

        Ok(tutor)
    }

    pub async fn update_is_available(tutor_id: &str, is_available: bool) -> Result<()> {
        let pool = db::connect_db().await;

        sqlx::query!(
            "UPDATE profile_management.tutor SET is_available = $1, updated_at = NOW() WHERE tutor_id = $2",
            is_available,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn update_city(tutor_id: &str, city: &str) -> Result<()> {
        let pool = db::connect_db().await;

        sqlx::query!(
            "UPDATE profile_management.tutor SET city = $1, updated_at = NOW() WHERE tutor_id = $2",
            city,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn update_country(tutor_id: &str, country: &str) -> Result<()> {
        let pool = db::connect_db().await;

        sqlx::query!(
            "UPDATE profile_management.tutor SET country = $1, updated_at = NOW() WHERE tutor_id = $2",
            country,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn update_year(tutor_id: &str, year: i32) -> Result<()> {
        let pool = db::connect_db().await;
        
        sqlx::query!(
            "UPDATE profile_management.tutor SET year = $1, updated_at = NOW() WHERE tutor_id = $2",
            year,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn update_bio(tutor_id: &str, bio: &str) -> Result<()> {
        let pool = db::connect_db().await;

        sqlx::query!(
            "UPDATE profile_management.tutor SET bio = $1, updated_at = NOW() WHERE tutor_id = $2",
            bio,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn update_average(tutor_id: &str, average: BigDecimal) -> Result<()> {
        let pool = db::connect_db().await;
        
        sqlx::query!(
            "UPDATE profile_management.tutor SET average = $1, updated_at = NOW() WHERE tutor_id = $2",
            average,
            tutor_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn delete(tutor_id: &str) -> Result<()> {
        let pool = db::connect_db().await;

        sqlx::query("DELETE FROM profile_management.tutor WHERE tutor_id = $1")
            .bind(tutor_id)
            .execute(&pool)
            .await?;

        Ok(())
    }
}