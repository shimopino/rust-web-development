use mini_redis::{client, Result as RedisResult};

// #[tokio::main]
pub async fn call() -> RedisResult<()> {
    // 指定されたアドレスに対して、非同期にTCPコネクションを確立する
    // コネクションが確立すれば client ハンドルを受け取る
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        call().await;
    });

    Ok(())
}
