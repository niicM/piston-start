use std::fs;
use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTexture {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atlas {
    pub width: i32,
    pub height: i32,
    pub image_path: String,
    #[serde(alias = "SubTexture")]
    pub sub_texture: Vec<SubTexture>
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    // Position
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,

    // Scale
    #[serde(default)]
    pub sc_x: f64,
    #[serde(default)]
    pub sc_y: f64,

    // Rotation
    #[serde(default)]
    pub sk_x: f64,
    #[serde(default)]
    pub sk_y: f64,
}

#[derive(Debug)]
pub struct Slot {
    pub name: String,
    pub texture_name: String,
    pub transform: Transform
}

#[derive(Debug)]
pub struct Character {
    pub slots: Vec<Slot>
}

impl Character {
    fn new() -> Self {
        Character {
            slots: Vec::new()
        }
    }
}

pub fn get_atlas(path: &std::path::Path) -> Atlas {
    let the_file = fs::read_to_string(path).expect("Unable to read atlas file");
    serde_json::from_str(&the_file).expect("JSON was not well-formatted")
}

pub fn get_character(path: &std::path::Path) -> Character {
    let the_file = fs::read_to_string(path).expect("Unable to read character file");
    let ske: Value = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    let mut character = Character::new();

    if let Value::Array(v) = &ske["armature"][0]["skin"][0]["slot"] {
        v.iter().for_each(|slot_read| {
            if let (Value::String(name), Value::String(texture_name)) =
                (&slot_read["name"], &slot_read["display"][0]["name"]) {

                let transform = serde_json::from_value(slot_read["display"][0]["transform"].clone()).unwrap();
                let slot = Slot {
                    name: name.clone(),
                    texture_name: texture_name.clone(),
                    transform: transform
                };
                character.slots.push(slot);
            }
        });
    }
    character
}