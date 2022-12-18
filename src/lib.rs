mod bingwallpaper;
mod image;
mod tooltips;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub use crate::{bingwallpaper::Bingwallpaper, image::Images, tooltips::Tooltips};
