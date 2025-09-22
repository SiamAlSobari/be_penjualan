use sqlx::MySqlPool;

use crate::model::profile_model;

pub struct ProfileRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> ProfileRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_user_name(
        &self,
        user_name: &str,
    ) -> Result<Option<profile_model::ResponseFindByUserName>, sqlx::Error> {
        let profile = sqlx::query_as!(
            profile_model::ResponseFindByUserName,
            "SELECT id,user_name FROM profiles WHERE user_name = ? LIMIT 1",
            user_name
        )
        .fetch_optional(self.pool)
        .await?;
        Ok(profile)
    }

    pub async fn insert_profile(&self, profile_id: &str,user_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO profiles (id,user_id) VALUES (?,?)",profile_id, user_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
