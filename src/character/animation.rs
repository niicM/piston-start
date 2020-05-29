use std::collections::HashMap;
use ai_behavior::{Behavior, Action, WhenAll, Sequence};
use sprite::{EaseFunction, Animation::{Ease, RotateBy, MoveBy}};
use serde::Deserialize;
use serde_json::Value;


#[derive(Debug, Deserialize)]
struct Translate {
    pub duration: f64,

    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

#[derive(Debug, Deserialize)]
struct Rotate {
    pub duration: f64,

    #[serde(default)]
    pub rotate: f64
}


pub struct Animations(Vec<(uuid::Uuid, Behavior<sprite::Animation>)>);

impl Animations {

    fn new() -> Animations {
        Animations(Vec::new())
    }

    pub fn play (&self, scene: &mut sprite::Scene<piston_window::Texture<gfx_device_gl::Resources>>) {
        for (id, anim) in &self.0 {
            scene.run(id.clone(), anim);
        }
    }

    fn from_dragon_rotation (json_val_vec:  Vec<Value>) -> Behavior<sprite::Animation> {
        let frames: Vec<Rotate> = json_val_vec.into_iter()
            .map(|f| serde_json::from_value::<Rotate>(f)
                .expect("Rotate animation with bad format"))
            .collect::<Vec<Rotate>>();

        let mut vector = Vec::<Behavior<sprite::Animation>>::new();

        let n = frames.len();
        for idx in 1..n {
            let duration = frames[idx - 1].duration;
            let rotate = frames[idx].rotate - frames[idx - 1].rotate;
            vector.push(Action(Ease(
                EaseFunction::CubicInOut,
                Box::new(RotateBy(duration, rotate)),
            )));
        }
        Sequence(vector)
    }

    pub fn from_dragon_translation (json_val_vec:  Vec<Value>) -> Behavior<sprite::Animation> {
        let frames: Vec<Translate> = json_val_vec.into_iter()
            .map(|f| serde_json::from_value::<Translate>(f)
                .expect("Rotate animation with bad format"))
            .collect::<Vec<Translate>>();

        let mut vector = Vec::<Behavior<sprite::Animation>>::new();

        let n = frames.len();
        for idx in 1..n {
            let duration = frames[idx - 1].duration;
            let x = frames[idx].x - frames[idx - 1].x;
            let y = frames[idx].y - frames[idx - 1].y;
            vector.push(Action(Ease(
                EaseFunction::CubicInOut,
                Box::new(MoveBy(duration, x, y)),
            )));
        }
        Sequence(vector)
    }

    pub fn from_dragon(json_val: Vec<Value>, id_map: &HashMap<super::Name, uuid::Uuid>) -> Animations {
        let mut anim = Animations::new();

        for bone_anim in json_val {

            // Should always match if json is well
            if let Some(Value::String(name)) = bone_anim.get("name") {
                let mut vector = Vec::<Behavior<sprite::Animation>>::new();

                if let Some(Value::Array(frames)) = bone_anim.get("translateFrame") {
                    vector.push(Animations::from_dragon_translation(frames.clone()));
                }
                if let Some(Value::Array(frames)) = bone_anim.get("rotateFrame") {
                    vector.push(Animations::from_dragon_rotation(frames.clone()));
                }

                anim.0.push((
                    id_map.get(&super::Name::BoneName(name.clone())).expect("No bone name").clone(), // Uuid
                    WhenAll(vector) // Rotation and translation
                ));
            }
        }
        anim
    }
}
