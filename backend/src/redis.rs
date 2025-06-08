//! This module provides functions to interact with Redis for storing and retrieving clipboard text.
pub mod rudis {

    const PREFIX: &str = "clipboard";
    fn build_redis_key(key: &str) -> String {
        format!("{}:{}", PREFIX, key)
    }

    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct RedisConfig {
        host: String,
        port: u16,
        db: u8,
        password: String,
    }

    fn connect() -> redis::Connection {
        use crate::utils::conf;
        let redis_conf = conf::read_table::<RedisConfig>("redis");

        // format: redis://:password@host:port/db
        let redis_conn_url = format!(
            "redis://:{}@{}:{}/{}",
            redis_conf.password, redis_conf.host, redis_conf.port, redis_conf.db
        );
        // println!("{}", redis_conn_url);

        redis::Client::open(redis_conn_url.as_str())
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis")
    }

    pub fn key_exists(key: &str) -> redis::RedisResult<bool> {
        let mut conn = connect();
        let redis_key = build_redis_key(key);
        let exists: bool = redis::cmd("EXISTS").arg(&redis_key).query(&mut conn)?;
        Ok(exists)
    }

    pub fn set_text(
        key: &str,
        value: &str,
        expire_time: usize,
        access_limit: u8,
    ) -> redis::RedisResult<()> {
        let mut conn = connect();
        let redis_key = build_redis_key(key);

        let _: () = redis::cmd("HSET")
            .arg(&redis_key)
            .arg("text")
            .arg(value)
            .arg("access_limit")
            .arg(access_limit.to_string())
            .query(&mut conn)
            .expect("failed to execute HSET");

        let _: () = redis::cmd("EXPIRE")
            .arg(redis_key)
            .arg(expire_time)
            .query(&mut conn)
            .expect("failed to set expire");

        Ok(())
    }

    pub fn get_text(key: &str) -> redis::RedisResult<Option<String>> {
        let mut conn = connect();
        let redis_key = build_redis_key(key);

        let (text_opt, access_limit_opt): (Option<String>, Option<String>) = redis::cmd("HMGET")
            .arg(&redis_key)
            .arg("text")
            .arg("access_limit")
            .query(&mut conn)?;

        let access_limit = access_limit_opt
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(0);

        if access_limit <= 1 {
            // clear the key
            let _: () = redis::cmd("DEL").arg(&redis_key).query(&mut conn)?;
        } else {
            // access_limit minus one
            let _: () = redis::cmd("HINCRBY")
                .arg(&redis_key)
                .arg("access_limit")
                .arg(-1)
                .query(&mut conn)?;
        }

        let text = match text_opt {
            Some(t) if !t.is_empty() && access_limit >= 1 => t,
            _ => return Ok(None),
        };

        Ok(Some(text))
    }
}
