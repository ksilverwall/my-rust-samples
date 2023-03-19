use crate::sender::PostedRecord;
use futures::executor::block_on;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PostStorageManager {
    pool: Pool<Postgres>,
}

impl PostStorageManager {
    pub fn new(pool: Pool<Postgres>) -> PostStorageManager {
        PostStorageManager { pool: pool }
    }
    pub fn load(&self) -> Vec<PostedRecord> {
        block_on(
            sqlx::query_as::<_, PostedRecord>(
                "SELECT id, user_name, posted_at, message FROM main.records",
            )
            .fetch_all(&self.pool),
        )
        .unwrap()
    }
    pub fn push(&self, user_id: &String, password: &String, message: &String) -> Result<(), String> {
        match block_on(
            sqlx::query_as::<_, NoRecord>(
                "INSERT INTO main.records (id, user_name, token, posted_at, message) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)"
            )
            .bind(Uuid::new_v4().as_bytes())
            .bind(user_id)
            .bind(password.as_bytes())
            .bind(message)
            .fetch_optional(&self.pool)
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("failed push to database: {e}")),
        }
    }
    pub fn delete(&self, user_id: &String, password: &String) {
        block_on(
            sqlx::query_as::<_, NoRecord>(
                "DELETE FROM main.records WHERE user_name = $1 AND token = $2",
            )
            .bind(user_id)
            .bind(password.as_bytes())
            .fetch_optional(&self.pool),
        )
        .unwrap();
    }
}

#[derive(sqlx::FromRow)]
struct NoRecord {}
