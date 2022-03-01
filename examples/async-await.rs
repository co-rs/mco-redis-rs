use mco_redis_rs::AsyncCommands;

#[tokio::main]
async fn main() -> mco_redis_rs::RedisResult<()> {
    let client = mco_redis_rs::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await?;

    con.set("key1", b"foo").await?;

    mco_redis_rs::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;

    let result = mco_redis_rs::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await;
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    Ok(())
}
