use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    println!("Weather for {}", city);

    Ok(())
}
