use blockfrost::{BlockFrostApi, Settings};

type MainResult = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult {
    let project_id = "YOUR_KEY_HERE";
    let api = BlockFrostApi::new(project_id, Settings::default());

    // let health = api.health().await.unwrap();
    // println!("{:?}", health);
    // dbg!(health.status());
    // println!("{:#?}", health.json::<Health>().await.unwrap());

    Ok(())
}
