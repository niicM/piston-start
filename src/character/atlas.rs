use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTexture {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atlas {
    pub width: i32,
    pub height: i32,
    pub image_path: String,
    #[serde(alias = "SubTexture")]
    pub sub_texture: Vec<SubTexture>,
}

impl Atlas {
    pub fn get_sub_texture(&self, name: &str) -> Option<&SubTexture> {
        for st in &self.sub_texture {
            if &st.name[..] == name {
                return Some(st);
            }
        }
        None
    }

    pub fn from_file(path: &std::path::Path) -> Atlas {
        let the_file = fs::read_to_string(path).expect("Unable to read atlas file");
        serde_json::from_str(&the_file).expect("JSON was not well-formatted")
    }
}
