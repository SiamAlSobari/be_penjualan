use sqlx::{MySql, MySqlPool, Transaction};

use crate::model::user_model;

pub struct UserRepository<'a> {
    pub pool: &'a MySqlPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<user_model::ResponseFindByEmail>, sqlx::Error> {
        let user: Option<user_model::ResponseFindByEmail> = sqlx::query_as!(
            user_model::ResponseFindByEmail,
            "SELECT id,email FROM users WHERE email = ? LIMIT 1",
            email
        )
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn insert_user(
        &self,
        tx: &mut Transaction<'_, MySql>,
        email: &str,
        hash_password: &str,
        user_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (id,email,hash_password) VALUES (?,?,?)",
            user_id,
            email,
            hash_password
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
