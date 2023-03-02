
#[tokio::main]
async fn main() -> eyre::Result<()> {

    let client = dagger_sdk::connect().await?;
    let context_dir = client
        .host()
        .directory("./app");

    let _reg = client
        .container()
        .build(context_dir.id().await?)
        // .publish(format!("docker.io/abigillu/bots-dagger"))
        .publish(format!("registry.digitalocean.com/diabhey/hivenetes/bots:dagger"))
        .await?;

    println!("Published image to: {:?}", _reg);

    Ok(())
}
