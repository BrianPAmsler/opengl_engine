#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use std::{cell::RefCell, rc::Rc};

use engine::{errors::{Error, Result}, game_object::{component::Component, ObjectID}, Engine};
use gl46::GL_BACK;
use gl_types::{geometric::normalize, vec2, vec3};
use glfw::Key;
use regex::Regex;

use crate::engine::{game_object::ComponentID, graphics::{Camera, Projection, gl_enums::{DepthFunction, EnableCap}, sprite_renderer::components::{Sprite, SpriteSheet}, terrain::Terrain}};


#[derive(Clone, Default)]
pub struct FPSCounter {
    count: i64,
    fixed_count: i64,
    last_update: f32,
    last_fixed_update: f32
}

impl Component for FPSCounter {
    fn update(&mut self, engine: &mut Engine, _: ObjectID, _: f32) -> Result<()> {
        if engine.input.get_key_state(Key::Escape).press {
            engine.gfx.set_should_close(true);
        }
        
        self.count += 1;
        let current_tick = engine.gfx.get_glfw_time() as f32;

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f32 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }

        Ok(())
    }

    fn fixed_update(&mut self, engine: &mut Engine, _: ObjectID, _: f32) -> Result<()> {

        self.fixed_count += 1;
        let current_tick = engine.gfx.get_glfw_time() as f32;

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
    camera: Rc<RefCell<Camera>>,
    camera_size: f32,
    sprite1: Option<ComponentID>,
    sprite2: Option<ComponentID>
}

impl Component for Renderer {
    fn init(&mut self, engine: &mut Engine, _: ObjectID) -> Result<()> {
        engine.gfx.glClearColor(0.75, 0.75, 0.75, 1.0);
        engine.gfx.__get_glfw_mut().set_swap_interval(glfw::SwapInterval::None);
        engine.gfx.glEnable(EnableCap::GL_CULL_FACE);
        engine.gfx.glEnable(EnableCap::GL_DEPTH_TEST);
        engine.gfx.glDepthFunc(DepthFunction::GL_GREATER);
        engine.gfx.glClearDepth(0.0);
        engine.gfx.glCullFace(GL_BACK);

        let sprite1 = engine.world.find_child(engine.world.get_root(), "Sprite 1")?.unwrap();
        let sprite2 = engine.world.find_child(engine.world.get_root(), "Sprite 2")?.unwrap();

        let sprite1 = engine.world.get_component::<Sprite>(sprite1)?;
        let sprite2 = engine.world.get_component::<Sprite>(sprite2)?;

        self.sprite1 = Some(sprite1);
        self.sprite2 = Some(sprite2);

        Ok(())
    }

    fn update(&mut self, engine: &mut Engine, _: ObjectID, delta_time: f32) -> Result<()> {
        let mut camera = self.camera.borrow_mut();
        let speed = 10.0;
        if engine.input.get_key_state(Key::W).is_down {
            let pos = camera.position();
            camera.set_position(pos + normalize(vec3!(1, 0, 1)) * delta_time * speed);
        }

        if engine.input.get_key_state(Key::A).is_down {
            let pos = camera.position();
            camera.set_position(pos + normalize(vec3!(-1, 0, 1)) * delta_time * speed);
        }
        if engine.input.get_key_state(Key::S).is_down {
            let pos = camera.position();
            camera.set_position(pos + normalize(vec3!(-1, 0, -1)) * delta_time * speed);
        }
        if engine.input.get_key_state(Key::D).is_down {
            let pos = camera.position();
            camera.set_position(pos + normalize(vec3!(1, 0, -1)) * delta_time * speed);
        }
        if engine.input.get_key_state(Key::Space).is_down {
            let pos = camera.position();
            camera.set_position(pos + vec3!(0, 1, 0) * delta_time * speed);
        }
        if engine.input.get_key_state(Key::LeftControl).is_down {
            let pos = camera.position();
            camera.set_position(pos + vec3!(0, -1, 0) * delta_time * speed);
        }

        let mut sprite = engine.world.borrow_component_mut::<Sprite>(self.sprite2.unwrap())?;
        if engine.input.get_key_state(Key::Up).is_down {
            sprite.data.position += vec3!(0, 0, 1) * delta_time * speed;
        }

        if engine.input.get_key_state(Key::Left).is_down {
            sprite.data.position += vec3!(-1, 0, 0) * delta_time * speed;
        }
        if engine.input.get_key_state(Key::Down).is_down {
            sprite.data.position += vec3!(0, 0, -1) * delta_time * speed;
        }
        if engine.input.get_key_state(Key::Right).is_down {
            sprite.data.position += vec3!(1, 0, 0) * delta_time * speed;
        }
        if engine.input.get_key_state(Key::RightShift).is_down {
            sprite.data.position += vec3!(0, 1, 0) * delta_time * speed;
        }
        if engine.input.get_key_state(Key::RightControl).is_down {
            sprite.data.position += vec3!(0, -1, 0) * delta_time * speed;
        }

        self.camera_size -= engine.input.get_scroll_y() as f32;

        match camera.projection_mut() {
            Projection::Orthographic { width, .. } => *width = self.camera_size,
            _ => ()
        }

        Ok(())   
    }
}

fn start_game() -> Result<()> {
    let mut engine = Engine::create_window("Test Window", 1280, 720, engine::WindowMode::Windowed)?;

    let a = engine.world.create_game_object("a", engine.world.get_root())?;

    let mut sprite_sheet = SpriteSheet::new("sprite_sheet.png");
    sprite_sheet.add_sprite(0, 0, 512, 512);
    sprite_sheet.add_sprite(512, 512, 1024, 1024);
    engine.world.add_component(a, sprite_sheet)?;
    
    let sprite1 = engine.world.create_game_object("Sprite 1", engine.world.get_root())?;
    let sprite2 = engine.world.create_game_object("Sprite 2", engine.world.get_root())?;

    let mut sprite_component1 = Sprite::new("sprite_sheet.png", 0);
    sprite_component1.data.anchor = vec2!(0.5, 0);
    let mut sprite_component2 = Sprite::new("sprite_sheet.png", 1);
    sprite_component2.data.anchor = vec2!(0.5, 0);

    engine.world.add_component(sprite1, sprite_component1)?;
    engine.world.add_component(sprite2, sprite_component2)?;
    
    let camera = Rc::new(RefCell::new(Camera::new(
        Projection::Orthographic {
            width: 1.0,
            aspect: 16.0 /9.0,
            z_near: -100.0,
            z_far: 100.0,
        },
        vec3!(0, 1, 0),
        normalize(vec3!(1, -1, 1)),
        vec3!(0, 1, 0)
    )));

    engine.world.set_main_camera(camera.clone());

    let terrain = Terrain::new("height_map.png", "ground.png");
    engine.world.add_component(a, terrain)?;

    let renderer = Renderer { camera_size: 10.0, camera, sprite1: None, sprite2: None  };

    engine.world.add_component(a, FPSCounter::default())?;
    engine.world.add_component(a, renderer)?;

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
