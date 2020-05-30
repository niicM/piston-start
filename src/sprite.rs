extern crate ai_behavior;
extern crate find_folder;
extern crate piston_window;
extern crate sprite;
extern crate serde;
extern crate serde_json;
extern crate graphics;
extern crate rand;
extern crate uuid;

use piston_window::PressEvent;

mod character;


fn main() {

    let (width, height) = (600, 600);
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

    let character = character::get_character(
        &mut scene,
        assets.clone(),
        String::from("robot"),
        &mut window
    ).expect("Problem creating character");

    let root = scene.child_mut(character.root_id).expect("Can't find sprite");
    root.set_position(400.0, 400.0);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

        if let Some(ev) = e.press_args() {
            match ev {
                _ => character.play(&mut scene)
            }
        }

    }
}
