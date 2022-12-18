use crate::{Images, Result, Tooltips};
use serde_derive::Deserialize;

const BING_URL: &str = "https://www.bing.com/HPImageArchive.aspx?&format=js";

#[derive(Debug)]
#[allow(dead_code)]
pub struct Bingwallpaper {
    index: u8,
    number: u8,
    files: Vec<String>,
    json: Wallpaper,
}

impl Bingwallpaper {
    pub async fn new(index: u8, number: u8) -> Result<Bingwallpaper> {
        let json = Wallpaper::new(index, number).await?;
        Ok(Bingwallpaper {
            index,
            number,
            files: vec![],
            json,
        })
    }

    pub async fn save_wallpaper(&self) -> Result<()> {
        self.json.images[0].save_wallpaper().await?;
        Ok(())
    }

    pub fn set_wallpaper(&self) {
        self.json.images[0].set_wallpaper();
    }
}

fn get_url(index: u8, number: u8) -> String {
    [
        BING_URL,
        "&idx=",
        &index.to_string(),
        "&n=",
        &number.to_string(),
    ]
    .join("")
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Wallpaper {
    pub images: Vec<Images>,
    pub tooltips: Tooltips,
}

impl Wallpaper {
    pub async fn new(index: u8, number: u8) -> Result<Wallpaper> {
        Ok(reqwest::get(get_url(index, number).as_str())
            .await?
            .json::<Wallpaper>()
            .await?)
    }
}
