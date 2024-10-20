#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use std::path::Path;

use engine::{errors::{Error, Result}, game_object::{component::Component, ObjectID}, graphics::{embed_shader_source, sprite_renderer::{SpriteData, SpriteRenderer}, BufferedMesh, CustomAttribute, CustomAttributeData, Graphics, Mesh, RGBColor, VBOBufferer, Vertex}, Engine};
use engine::graphics::{VertexShader, FragmentShader, ShaderProgram, ShaderProgramBuilder};
use gl46::{GL_COLOR_BUFFER_BIT, GL_TRIANGLES};
use glm::Mat4;
use image::ImageReader;
use regex::Regex;

fn load_texture<P: AsRef<Path>>(path: P, buffer: &mut Vec<u8>) -> Result<(u32, u32)> {
    let img = ImageReader::open(path).map_err(|t| t.to_string())?.decode().map_err(|t| t.to_string())?;
    let dimensions = (img.width(), img.height());
    buffer.clone_from_slice(img.as_bytes());

    Ok(dimensions)
}

#[derive(Clone, Default)]
pub struct FPSCounter {
    count: i64,
    fixed_count: i64,
    last_update: f32,
    last_fixed_update: f32
}

impl Component for FPSCounter {
    fn update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<()> {
        self.count += 1;
        let current_tick = _graphics.get_glfw_time() as f32;

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f32 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }

        Ok(())
    }

    fn fixed_update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<()> {
        self.fixed_count += 1;
        let current_tick = _graphics.get_glfw_time() as f32;

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
    sprite_renderer: SpriteRenderer
}

impl Component for Renderer {
    fn init(&mut self, _graphics: &Graphics, _owner: ObjectID) -> Result<()> {
        self.sprite_renderer.add_sprite(0, 0, 512, 512);
        self.sprite_renderer.add_sprite(512, 512, 1024, 1024);

        // // let mat = 
        // self.sprite_renderer.update_view_matrix(view_matrix);

        Ok(())
    }

    fn update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<()> {
        self.sprite_renderer.queue_sprite_instance(SpriteData {
            position: vec3!(0),
            anchor: vec2!(0.5, 0.5),
            dimensions: vec2!(1),
            sprite_id: 0,
        });
        self.sprite_renderer.render(_graphics);

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

    let mut sprite_map_data = Vec::new();
    let (width, height) = load_texture("sprite_sheet.png", &mut sprite_map_data)?;
    let sprite_renderer = SpriteRenderer::new(gfx, 1024, &sprite_map_data[..], width, height)?;
    let renderer = Renderer { sprite_renderer };

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
