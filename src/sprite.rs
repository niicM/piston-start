extern crate ai_behavior;
extern crate find_folder;
extern crate piston_window;
extern crate sprite;
extern crate serde;
extern crate graphics;
extern crate rand;
extern crate uuid;

use piston_window::PressEvent;
use std::rc::Rc;
use rand::prelude::*;

mod subtextures {

    use std::fs;
    use serde::Deserialize;

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


    pub fn get_atlas (atlas_path: &std::path::Path) -> Atlas {
        let the_file = fs::read_to_string(atlas_path).expect("Unable to read file");
        let atlas: Atlas = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
        return atlas;
    }
}

fn main() {
    let texture_name = "Rooster_Ani_tex";

    let (width, height) = (300, 300);
    let opengl = piston_window::OpenGL::V3_2;
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut scene = sprite::Scene::new();
//    let mut scene: sprite::Scene<graphics::image::Image> = sprite::Scene::new();

    let mut texture_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let atlas_path_buff = assets.join(texture_name.to_string() + &".json".to_string());
    let atlas_path = atlas_path_buff.as_path();
    let atlas = subtextures::get_atlas(atlas_path);

    let tex = Rc::new(
        piston_window::Texture::from_path(
            &mut texture_context,
            assets.join(texture_name.to_string() + &".png".to_string()),
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        ).unwrap()
    );

    let mut sprite_ids = Vec::new();

    atlas.sub_texture.iter().for_each(
        |t| sprite_ids.push(
            scene.add_child(
                sprite::Sprite::from_texture_rect(
                    tex.clone(),
                    [t.x.into(), t.y.into(), t.width.into(), t.height.into()]))));

//    let mut rng = rand::prelude::thread_rng();
//    for id in &sprite_ids {
//        scene.child_mut(id.clone())
//            .expect("No sprite")
//            .set_position(
//                rng.gen_range(0.0, 300.0) as f64,
//                rng.gen_range(0.0, 300.0) as f64);
//    }

    randomize_sprites(&mut scene, &mut sprite_ids);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

        if let Some(ev) = e.press_args() {
            use piston_window::Button::*;
            match ev {
                Keyboard(_) => randomize_sprites(&mut scene, &mut sprite_ids),
                _ => move_sprites_around(&mut scene, &mut sprite_ids),
            }
        }

    }
}


fn randomize_sprites<I: gfx_core::Resources>(scene: &mut sprite::Scene<piston_window::Texture<I>>, sprite_ids: &Vec<uuid::Uuid>) {
    let mut rng = rand::prelude::thread_rng();
    for id in sprite_ids {
        scene.child_mut(id.clone())
            .expect("No sprite")
            .set_position(
                rng.gen_range(0.0, 300.0) as f64,
                rng.gen_range(0.0, 300.0) as f64);
    }
}


fn move_sprites_around<I: gfx_core::Resources>(scene: &mut sprite::Scene<piston_window::Texture<I>>, sprite_ids: &Vec<uuid::Uuid>) {
    let mut rng = rand::prelude::thread_rng();
    for id in sprite_ids {
        let action = ai_behavior::Action(sprite::Animation::Ease(
            sprite::EaseFunction::ExponentialInOut,
            Box::new(sprite::Animation::RotateTo(2.0, rng.gen_range(0.0, 360.0))),
        ));
        scene.run(id.clone(), &action);
    }
}