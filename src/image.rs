use crate::Result;
use serde_derive::Deserialize;
use serde_json::Number;
use std::{
    env::var,
    fs,
    io::{copy, Cursor},
    path::Path,
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Images {
    pub bot: Number,
    pub copyright: String,
    pub copyrightlink: String,
    pub drk: Number,
    pub enddate: String,
    pub fullstartdate: String,
    pub hs: Vec<String>,
    pub hsh: String,
    pub quiz: String,
    pub startdate: String,
    pub title: String,
    pub top: Number,
    pub url: String,
    pub urlbase: String,
    pub wp: bool,
}

impl Images {
    pub fn url(&self) -> String {
        ["https://www.bing.com", &self.url].concat()
    }

    fn filename(&self) -> &str {
        let s = self.url.find("OHR.").ok_or(0).unwrap();
        let e = self.url.find("&rf=").ok_or(0).unwrap();
        &self.url[s..e]
    }

    fn copyright(&self) -> &str {
        self.copyright.as_str()
    }

    fn directory(&self) -> String {
        [var("HOME").unwrap_or_default().as_str(), "/Pictures/Bing"].concat()
    }

    pub async fn save_wallpaper(&self) -> Result<()> {
        fs::create_dir_all(self.directory())?;
        let res = reqwest::get(self.url().as_str()).await?;
        copy(
            &mut Cursor::new(res.bytes().await?),
            &mut fs::File::create(Path::new(&self.directory()).join(self.filename()))?,
        )?;
        Ok(())
    }

    pub fn set_wallpaper(&self) {
        let file = format!("{}/{}", self.directory(), self.filename());
        println!("{}", file);
        println!("{}", self.copyright());
        wallpaper::set_from_path(&file).unwrap();
    }
}
