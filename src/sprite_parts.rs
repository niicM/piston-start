extern crate ai_behavior;
extern crate find_folder;
extern crate piston_window;
extern crate sprite;
extern crate serde;
extern crate graphics;

mod subtextures {
    use graphics;
    use std::rc::Rc;
    use std::fs;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SubTexture {
        name: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Atlas {
        width: i32,
        height: i32,
        image_path: String,
        #[serde(alias = "SubTexture")]
        sub_texture: Vec<SubTexture>
    }


    pub fn get_atlas<I: graphics::ImageSize, F, R, C>(path: std::path::PathBuf, name: String, &mut texture_context: piston_window::TextureContext<F, R, C>)
                     -> Vec<sprite::Sprite<piston_window::Texture<I>>> {
        let tex = Rc::new(
            piston_window::Texture::from_path(
                &mut texture_context,
                path.join(name + ".png"),
                piston_window::Flip::None,
                &piston_window::TextureSettings::new(),
            ).unwrap(),
        );
        let the_file = fs::read_to_string(path.join(name + ".json")).expect("Unable to read file");
        let atlas: Atlas = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
        let textures = Vec::new();
        atlas.sub_texture.iter().for_each(
            |tex| textures.push(sprite::Sprite::from_texture_rect(tex.clone(), [tex.x, tex.y, tex.width, tex.height])));
        return textures;
    }
}

fn main() {
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
    let id;
    let mut scene = sprite::Scene::new();
    let mut texture_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mut textures = subtextures::get_atlas(assets, "Rooster_Ani_tex".to_string(), texture_context);
    let &mut sprite = textures[0];
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    id = scene.add_child(sprite);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });
    }
}
