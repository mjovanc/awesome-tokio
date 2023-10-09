use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("awesome", "tokio".into()).await?;

    let result = client.get("awesome").await?;

    println!("got an awesome value from the server; result={:?}", result);

    Ok(())
}