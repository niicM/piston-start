use serde::Deserialize;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use piston_window::PistonWindow;
use sprite::Sprite;


pub mod atlas;
pub mod animation;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    // Position
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,

    // Scale
    #[serde(default = "default_scale")]
    pub sc_x: f64,
    #[serde(default = "default_scale")]
    pub sc_y: f64,

    // Rotation
    #[serde(default)]
    pub sk_x: f64
}

impl Transform {
    fn empty() -> Transform {
        Transform { x: 0.0, y: 0.0, sc_x: 1.0, sc_y: 1.0, sk_x: 0.0 }
    }
}

fn default_scale() -> f64 {
    1.0
}

#[derive(Debug)]
pub struct Slot {
    pub name: String,
    //    pub parent: String,
    pub texture_name: String,
    pub sprite_id: uuid::Uuid,
    pub transform: Transform,
}

#[derive(Debug)]
pub struct Character {
    pub slots: Vec<Slot>,
}

impl Character {
    fn new() -> Self {
        Character { slots: Vec::new() }
    }
}

// The same name can be reused in different contexts
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Name {
    SlotName(String),
    BoneName(String)
}


//gfx_core::Resources
pub fn get_character_2<W: piston_window::OpenGLWindow>(
    scene: &mut sprite::Scene<piston_window::Texture<gfx_device_gl::Resources>>,
    path: std::path::PathBuf,
    character_name: String,
    window: &mut PistonWindow<W>) -> Option<animation::Animations>
{
    // Setup paths for different files
    let path_ske_j = path.join(character_name.clone() + "_ske.json");
    let path_tex_j = path.join(character_name.clone() + "_tex.json");
    let path_tex_p = path.join(character_name + "_tex.png");
    let path_bone_p = path.join("bone_tex.png");

    let my_atlas = atlas::Atlas::from_file(path_tex_j.as_path());

    // Textures
    let mut texture_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let tex = Rc::new(
        piston_window::Texture::from_path(
            &mut texture_context,
            path_tex_p,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        ).unwrap()
    );
    let tex_bone = Rc::new(
        piston_window::Texture::from_path(
            &mut texture_context,
            path_bone_p,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        ).unwrap()
    );

    let the_ske_file = fs::read_to_string(path_ske_j).expect("Unable to read character file");
    let ske: Value = serde_json::from_str(&the_ske_file).expect("JSON was not well-formatted");

    let mut sprite_map = HashMap::new();

    // We need a way to refer to the Sprites while moving them to the Scene or into other Sprites
    let mut sprite_ids: HashMap<Name, uuid::Uuid> = HashMap::new();

    // sprites SubTexture and transform
    if let Value::Array(slots) = &ske["armature"][0]["skin"][0]["slot"] {
        for slot in slots {
            if let (Value::String(name), Value::String(texture_name)) =
                   (&slot["name"], &slot["display"][0]["name"]) {
                let t = my_atlas.get_sub_texture(&texture_name[..]).unwrap();
                let mut sprite = Sprite::from_texture_rect(
                    tex.clone(),
                    [t.x.clone().into(), t.y.clone().into(), t.width.clone().into(), t.height.clone().into()]);

                let transform = serde_json::from_value::<Transform>(slot["display"][0]["transform"].clone()).unwrap();
                sprite.set_position(transform.x.clone(), transform.y.clone());
                sprite.set_rotation(transform.sk_x.clone());
                sprite.set_scale(transform.sc_x.clone(), transform.sc_y.clone());

                sprite_ids.insert(Name::SlotName(name.clone()), sprite.id());
                sprite_map.insert(Name::SlotName(name.clone()), sprite);
            }
        }
    }


    // Bone name, (parent) and transform
    if let Value::Array(bones) = &ske["armature"][0]["bone"] {
        for bone in bones {
            if let Some(Value::String(name)) = &bone.get("name") {
                let mut sprite = Sprite::from_texture(tex_bone.clone());
                let transform = match bone.get("transform") {
                    Some(t) => serde_json::from_value::<Transform>(t.clone()).unwrap(),
                    None => Transform::empty()
                };
                sprite.set_position(transform.x.clone(), transform.y.clone());
                sprite.set_rotation(transform.sk_x.clone());
                sprite.set_scale(transform.sc_x.clone(), transform.sc_y.clone());
                sprite.set_anchor(0.0, 0.0);

                sprite_ids.insert(Name::BoneName(name.clone()), sprite.id());
                sprite_map.insert(Name::BoneName(name.clone()), sprite);
            }
        }
    }

    // root is the only sprite that is directly in the scene
    // The root object is not explicit in the json
    let root = Sprite::from_texture(tex_bone.clone());
    sprite_ids.insert(Name::BoneName("root".to_string()), scene.add_child(root));

    // Parent slots
    if let Value::Array(slots) = &ske["armature"][0]["slot"] {
        for slot in slots {
            if let (Some(Value::String(name)), Some(Value::String(parent))) =
                   (&slot.get("name"), &slot.get("parent")) {

                let parent_name = Name::BoneName(parent.clone());
                let child_name = Name::SlotName(name.clone());

                let parent_id = sprite_ids.get(&parent_name)
                    .expect("Unknown parent name");

                let child_sprite = sprite_map.remove(&child_name)
                    .expect("Child not present");

                let parent_sprite =
                    scene.child_mut(parent_id.clone()).or(sprite_map.get_mut(&parent_name))
                        .expect("Can't find parent sprite");

                parent_sprite.add_child(child_sprite);
            }
        }
    }

    // Parent bones
    // Bone (name), parent and (transform)
    if let Value::Array(bones) = &ske["armature"][0]["bone"] {
        for bone in bones {
            if let (Some(Value::String(name)), Some(Value::String(parent))) =
                   (bone.get("name"), bone.get("parent")) {

                let parent_name = Name::BoneName(parent.clone());
                let child_name = Name::BoneName(name.clone());

                let parent_id = sprite_ids.get(&parent_name)
                    .expect("Unknown parent name");

                let child_sprite = sprite_map.remove(&child_name)
                    .expect("Child not present");

                let parent_sprite =
                    scene.child_mut(parent_id.clone()).or(sprite_map.get_mut(&parent_name))
                        .expect("Can't find parent sprite");

                parent_sprite.add_child(child_sprite);
            }
        }
    }

    if let Value::Array(json_val) = &ske["armature"][0]["animation"][0]["bone"] {
        return Some(animation::Animations::from_dragon(json_val.clone(), &sprite_ids))
    }
    None
}





pub fn get_character(path: &std::path::Path, id_map: &HashMap<String, uuid::Uuid>) -> Character {
    let the_file = fs::read_to_string(path).expect("Unable to read character file");
    let ske: Value = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    let mut character = Character::new();

    if let Value::Array(v) = &ske["armature"][0]["skin"][0]["slot"] {
        v.iter().for_each(|slot_read| {
            if let (Value::String(name), Value::String(texture_name)) =
                (&slot_read["name"], &slot_read["display"][0]["name"])
            {
                let transform =
                    serde_json::from_value(slot_read["display"][0]["transform"].clone()).unwrap();
                let slot = Slot {
                    name: name.clone(),
                    texture_name: texture_name.clone(),
                    sprite_id: id_map
                        .get(texture_name)
                        .expect("Can't find texture id")
                        .clone(),
                    transform,
                };
                character.slots.push(slot);
            }
        });
    }
    character
}
