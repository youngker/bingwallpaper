use bingwallpaper::{Bingwallpaper, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let bingwallpaper = Bingwallpaper::new(0, 1).await?;
    bingwallpaper.save_wallpaper().await?;
    bingwallpaper.set_wallpaper();
    Ok(())
}
