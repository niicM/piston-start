extern crate ai_behavior;
extern crate find_folder;
extern crate piston_window;
extern crate sprite;
extern crate serde;
extern crate graphics;

use std::rc::Rc;

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

    let mut textures = Vec::new();

    atlas.sub_texture.iter().for_each(
        |t| textures.push(
            scene.add_child(
                sprite::Sprite::from_texture_rect(
                    tex.clone(),
                    [t.x.into(), t.y.into(), t.width.into(), t.height.into()]))));


//    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });
    }
}
