// Uses https://crates.io/crates/dagger-sdk for ci

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;
    let context_dir = client.host().directory("./pkg/app");

    let _reg = client
        .container()
        .build(context_dir.id().await?)
        .publish("registry.digitalocean.com/diabhey/hivenetes/bots:dagger".to_string())
        .await?;

    println!("Published image to: {:?}", _reg);

    Ok(())
}
