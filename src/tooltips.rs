use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Tooltips {
    pub loading: String,
    pub next: String,
    pub previous: String,
    pub walle: String,
    pub walls: String,
}
