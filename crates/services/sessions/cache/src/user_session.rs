use chrono::{DateTime, NaiveDateTime, Utc};
use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};
use uuid::Uuid;

pub struct UserSession {
    pub key: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl UserSession {
    pub fn check_timeout(&mut self, ctx: &Context) -> RedisResult {
        let key = RedisString::create(None, self.key.clone());
        let _key = ctx.open_key_writable(&key);
        unimplemented!()
    }

    pub fn get_counter(&mut self, ctx: &Context) -> RedisResult { unimplemented!() }

    pub fn update_last_interated(&mut self, ctx: &Context) -> RedisResult { unimplemented!() }
}

impl From<Uuid> for UserSession {
    fn from(id: Uuid) -> Self {
        Self {
            key: format!("user_session_{id}"),
            user_id: id,
            created_at: Utc::now(),
        }
    }
}
