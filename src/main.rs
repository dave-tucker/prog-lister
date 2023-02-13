use aya::programs::programs;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    for p in programs() {
        println!("{}", String::from_utf8_lossy(p.unwrap().name()))
    }

    Ok(())
}
