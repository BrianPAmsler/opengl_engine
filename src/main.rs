#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use std::path::Path;

use engine::{errors::{Error, Result}, game_object::{component::Component, ObjectID, World}, graphics::{image::Image, sprite_renderer::{SpriteData, SpriteRenderer}, Graphics}, input::Input, Engine};
use gl46::{GL_BACK, GL_COLOR_BUFFER_BIT, GL_CULL_FACE};
use gl_types::{angle_trig::radians, clip_space::perspective, matrix::inverse, transform::lookAt, vec2, vec3, vectors::Vec3};
use glfw::Key;
use image::ImageReader;
use regex::Regex;


#[derive(Clone, Default)]
pub struct FPSCounter {
    count: i64,
    fixed_count: i64,
    last_update: f32,
    last_fixed_update: f32
}

impl Component for FPSCounter {
    fn update(&mut self, gfx: &Graphics, _: &World, _: ObjectID, _: f32, _: &Input) -> Result<()> {
        self.count += 1;
        let current_tick = gfx.get_glfw_time() as f32;

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f32 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }

        Ok(())
    }

    fn fixed_update(&mut self, gfx: &Graphics, _: &World, _: ObjectID, _: f32, input: &Input) -> Result<()> {
        if input.get_key_state(Key::Escape).press {
            gfx.set_should_close(true);
        }

        self.fixed_count += 1;
        let current_tick = gfx.get_glfw_time() as f32;

        let delta = current_tick - self.last_fixed_update;

        if delta >= 1.0 {
            let fps = self.fixed_count as f32 / delta;
            println!("Fixed FPS: {}\n", fps);

            self.fixed_count = 0;
            self.last_fixed_update = current_tick;
        }

        Ok(())
    }
}

pub struct Renderer {
    sprite_renderer: SpriteRenderer,
    position: Vec3
}

impl Component for Renderer {
    fn init(&mut self, gfx: &Graphics, _: &World, _: ObjectID) -> Result<()> {
        gfx.glClearColor(0.75, 0.75, 0.75, 1.0);
        self.sprite_renderer.add_sprite(0, 0, 512, 512);
        self.sprite_renderer.add_sprite(512, 512, 1024, 1024);
        self.sprite_renderer.add_sprite(512, 512, 1024, 1024);

        gfx.__get_glfw_mut().set_swap_interval(glfw::SwapInterval::None);

        // self.sprite_renderer.update_projection_matrix(ortho(-2.0, 2.0, -1.5, 1.5, 0.0, 100.0));
        self.sprite_renderer.update_projection_matrix(perspective(radians(90.0), 2.0 / 1.5, 0.1, 100.0));

        self.sprite_renderer.update_sprite_map(gfx);
        gfx.glEnable(GL_CULL_FACE);
        gfx.glCullFace(GL_BACK);

        Ok(())
    }

    fn update(&mut self, gfx: &Graphics, _: &World, _: ObjectID, delta_time: f32, input: &Input) -> Result<()> {
        let speed = 1.0;
        if input.get_key_state(Key::W).is_down {
            self.position += vec3!(0, 0, -1) * delta_time * speed;
        }

        if input.get_key_state(Key::A).is_down {
            self.position += vec3!(-1, 0, 0) * delta_time * speed;
        }
        if input.get_key_state(Key::S).is_down {
            self.position += vec3!(0, 0, 1) * delta_time * speed;
        }
        if input.get_key_state(Key::D).is_down {
            self.position += vec3!(1, 0, 0) * delta_time * speed;
        }

        let mat = lookAt(self.position, self.position + vec3!(0, 0, -1), vec3!(0, 1, 0));

        self.sprite_renderer.update_view_matrix(inverse(mat));

        gfx.glClear(GL_COLOR_BUFFER_BIT);
        self.sprite_renderer.queue_sprite_instance(SpriteData {
            position: vec3!(0, 0, 0),
            anchor: vec2!(0, 0),
            dimensions: vec2!(1),
            sprite_id: 1,
        });
        self.sprite_renderer.queue_sprite_instance(SpriteData {
            position: vec3!(2, 0, 0),
            anchor: vec2!(0, 0),
            dimensions: vec2!(2),
            sprite_id: 0,
        });
        self.sprite_renderer.render(gfx);

        Ok(())   
    }
}

fn start_game() -> Result<()> {
    let mut engine = Engine::new()?;
    engine.create_window("Test Window", 800, 600, engine::WindowMode::Windowed)?;

    let world = engine.get_world();

    let a = world.create_game_object("a".to_owned(), world.get_root())?;
    let _b = world.create_game_object("b".to_owned(), a)?;
    let c = world.create_game_object("c".to_owned(), a)?;
    let _d = world.create_game_object("d".to_owned(), c)?;
    
    let gfx = engine.get_graphics()?;

    let sprite_map = Image::load_from_file("sprite_sheet.png")?;

    let sprite_renderer = SpriteRenderer::new(gfx, 1024, sprite_map)?;
    let renderer = Renderer { sprite_renderer, position: vec3!(0, 0, 1) };

    let world = engine.get_world();

    world.add_component(_d, FPSCounter::default())?;
    world.add_component(a, renderer)?;

    engine.run()?;

    Ok(())
}

fn main() {
    match start_game() {
        Ok(_) => {},
        Err(err) => { eprint!("{}", clean_backtrace(&err, "opengl_engine")); }
    }
}

pub fn clean_backtrace(error: &Error, crate_name: &'static str) -> String {
    let str = format!("{:?}", error.backtrace());

    let mut clean_str = String::new();
    clean_str.reserve(str.len());

    clean_str += &format!("Error: {}\n\nStack Backtrace\n", error.to_string());
    
    let is_error_line = Regex::new("^ +[0-9]+:").unwrap();
    let in_crate = Regex::new(&format!("^ +[0-9]+: {}::", crate_name)).unwrap();

    let mut count = 0;
    let mut adding = false;
    for line in str.split('\n') {
        let result = is_error_line.find(line);

        if adding {
            if result.is_some() {
                adding = false;
            } else {
                clean_str += &line;
                clean_str += "\n";
            }
        }
        if !adding {
            match result {
                Some(line_number) => {
                    if in_crate.find(line).is_some() {
                        adding = true;
                        
                        let new_line = format!("   {}: ", count) + &line[line_number.end()..];
                        clean_str += &new_line;
                        clean_str += "\n";
        
                        count += 1;
                    }
                },
                None => {}
            }
        }
    }

    clean_str
}
