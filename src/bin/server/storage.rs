use crate::sender::PostedRecord;
use futures::executor::block_on;
use sqlx::{Pool, Postgres};

pub struct PostStrageManager {
    pool: Pool<Postgres>,
}

impl PostStrageManager {
    pub fn new(pool: Pool<Postgres>) -> PostStrageManager {
        PostStrageManager { pool: pool }
    }
    pub fn load(&self) -> Vec<PostedRecord> {
        block_on(
            sqlx::query_as::<_, PostedRecord>(
                "SELECT user_name, posted_at, message FROM main.records",
            )
            .fetch_all(&self.pool),
        )
        .unwrap()
    }
    pub fn push(&self, user_id: &String, password: &String, message: &String) {
        block_on(
        sqlx::query_as::<_, NoRecord>(
            "INSERT INTO main.records (user_name, token, posted_at, message) VALUES ($1, $2, CURRENT_TIMESTAMP, $3)"
        )
        .bind(user_id)
        .bind(password.as_bytes())
        .bind(message)
        .fetch_optional(&self.pool)
    )
    .unwrap();
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
