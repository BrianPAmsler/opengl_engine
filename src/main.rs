#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use engine::{errors::{Error, Result}, game_object::{component::Component, ObjectID, World}, graphics::{image::Image, sprite_renderer::{SpriteData, SpriteRenderer}, Graphics}, input::Input, Engine};
use gl46::{GL_BACK, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
use gl_types::{angle_trig::radians, clip_space::{ortho, ortho_aspect, perspective}, geometric::normalize, matrices::Mat4, transform::lookAt, vec2, vec3, vectors::Vec3};
use glfw::Key;
use image::{ImageBuffer, Luma, imageops};
use regex::Regex;

use crate::engine::graphics::{gl_enums::{DepthFunction, EnableCap}, terrain::{Terrain, terrain_renderer::{TerrainRenderer}}};


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
    terrain_renderer: TerrainRenderer,
    terrain: Terrain,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    position: Vec3,
    sprite_position: Vec3,
    rot_y: f32,
    rot_x: f32,
    camera_size: f32
}

impl Component for Renderer {
    fn init(&mut self, gfx: &Graphics, _: &World, _: ObjectID) -> Result<()> {
        gfx.glClearColor(0.75, 0.75, 0.75, 1.0);
        self.sprite_renderer.add_sprite(0, 0, 512, 512);
        self.sprite_renderer.add_sprite(512, 512, 1024, 1024);
        self.sprite_renderer.add_sprite(512, 512, 1024, 1024);

        gfx.__get_glfw_mut().set_swap_interval(glfw::SwapInterval::None);

        self.projection_matrix  = ortho_aspect(10.0, 16.0 / 9.0, -100.0, 100.0);
        // self.projection_matrix = perspective(radians(90.0), 16.0/9.0, 0.1, 1000.0);

        self.sprite_renderer.update_sprite_map(gfx);
        gfx.glEnable(EnableCap::GL_CULL_FACE);
        gfx.glEnable(EnableCap::GL_DEPTH_TEST);
        gfx.glDepthFunc(DepthFunction::GL_GREATER);
        gfx.glClearDepth(0.0);
        gfx.glCullFace(GL_BACK);

        Ok(())
    }

    fn update(&mut self, gfx: &Graphics, _: &World, _: ObjectID, delta_time: f32, input: &Input) -> Result<()> {
        let speed = 10.0;
        if input.get_key_state(Key::W).is_down {
            self.position += normalize(vec3!(1, 0, 1)) * delta_time * speed;
        }

        if input.get_key_state(Key::A).is_down {
            self.position += normalize(vec3!(-1, 0, 1)) * delta_time * speed;
        }
        if input.get_key_state(Key::S).is_down {
            self.position += normalize(vec3!(-1, 0, -1)) * delta_time * speed;
        }
        if input.get_key_state(Key::D).is_down {
            self.position += normalize(vec3!(1, 0, -1)) * delta_time * speed;
        }
        if input.get_key_state(Key::Space).is_down {
            self.position += vec3!(0, 1, 0) * delta_time * speed;
        }
        if input.get_key_state(Key::LeftControl).is_down {
            self.position += vec3!(0, -1, 0) * delta_time * speed;
        }

        if input.get_key_state(Key::Up).is_down {
            self.sprite_position += vec3!(0, 0, 1) * delta_time * speed;
        }

        if input.get_key_state(Key::Left).is_down {
            self.sprite_position += vec3!(-1, 0, 0) * delta_time * speed;
        }
        if input.get_key_state(Key::Down).is_down {
            self.sprite_position += vec3!(0, 0, -1) * delta_time * speed;
        }
        if input.get_key_state(Key::Right).is_down {
            self.sprite_position += vec3!(1, 0, 0) * delta_time * speed;
        }
        if input.get_key_state(Key::RightShift).is_down {
            self.sprite_position += vec3!(0, 1, 0) * delta_time * speed;
        }
        if input.get_key_state(Key::RightControl).is_down {
            self.sprite_position += vec3!(0, -1, 0) * delta_time * speed;
        }

        if input.get_key_state(Key::Kp9).press {
            self.camera_size += 5.0;
        }
        self.camera_size -= input.get_scroll_y() as f32;

        if input.get_key_state(Key::Q).is_down {
            self.rot_y -= 0.5 * delta_time;
        }

        if input.get_key_state(Key::E).is_down {
            self.rot_y += 0.5 * delta_time;
        }

        let change = if input.get_key_state(Key::LeftShift).is_down {
            -1
        } else {
            1
        };

        self.projection_matrix  = ortho_aspect(self.camera_size, 16.0 / 9.0, -100.0, 100.0);
        // let mut cell = self.terrain.get_cell_mut(2, 2).unwrap();
        // if input.get_key_state(Key::Kp1).press {
        //     *cell.bottom_left = cell.bottom_left.saturating_add_signed(change);
        // }
        // if input.get_key_state(Key::Kp2).press {
        //     *cell.bottom_right = cell.bottom_right.saturating_add_signed(change);
        // }
        // if input.get_key_state(Key::Kp4).press {
        //     let mut top_left = cell.top_left();
        //     *top_left.height() = top_left.height().saturating_add_signed(change);
        // }
        // if input.get_key_state(Key::Kp5).press {
        //     *cell.top_right = cell.top_right.saturating_add_signed(change);
        // }

        // println!("Cell:\t{}, {}\n\t{}, {}", cell.top_left, cell.top_right, cell.bottom_left, cell.bottom_right);

        let offset = vec3!(f32::sin(self.rot_y), 0 , f32::cos(self.rot_y));
        let mat = lookAt(self.position, self.position + vec3!(1, -1, 1), vec3!(0, 1, 0));

        self.view_matrix = mat;

        gfx.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        self.sprite_renderer.queue_sprite_instance(SpriteData {
            position: vec3!(0, 0.5, 0),
            anchor: vec2!(0.5, 0),
            dimensions: vec2!(1),
            sprite_id: 1,
        });
        self.sprite_renderer.queue_sprite_instance(SpriteData {
            position: self.sprite_position,
            anchor: vec2!(0.5, 0),
            dimensions: vec2!(2),
            sprite_id: 0,
        });
        self.terrain_renderer.render(gfx, &mut self.terrain, self.view_matrix, self.projection_matrix, self.position);
        self.sprite_renderer.render(gfx, &self.view_matrix, &self.projection_matrix);

        Ok(())   
    }
}

fn start_game() -> Result<()> {
    let mut engine = Engine::new()?;
    engine.create_window("Test Window", 2560, 1440, engine::WindowMode::FullScreen(None))?;

    let world = engine.get_world();

    let a = world.create_game_object("a".to_owned(), world.get_root())?;
    let _b = world.create_game_object("b".to_owned(), a)?;
    let c = world.create_game_object("c".to_owned(), a)?;
    let _d = world.create_game_object("d".to_owned(), c)?;
    
    let gfx = engine.get_graphics()?;

    let sprite_map = Image::load_from_file("sprite_sheet.png")?;
    let grid = image::ImageReader::open("ground.png")?.decode()?;
    let mut grid = grid.to_rgb8();
    imageops::flip_vertical_in_place(&mut grid);

    let height_map = image::ImageReader::open("height_map.png")?.decode()?;
    let height_map = height_map.to_rgb8();
    let (width, height) = height_map.dimensions();
    let height_map: Vec<u8> = height_map.into_raw().into_iter().step_by(3).collect();
    let mut height_map: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, height_map).unwrap();
    imageops::flip_vertical_in_place(&mut height_map);

    let sprite_renderer = SpriteRenderer::new(gfx, 1024, sprite_map)?;
    let terrain_renderer = TerrainRenderer::new(gfx)?;
    let terrain = Terrain::from_raw(gfx, height_map.into_raw().into_boxed_slice(), grid.into_raw().into_boxed_slice(), 200, 200);
    let renderer = Renderer { sprite_renderer, terrain_renderer, camera_size: 10.0, terrain, position: vec3!(-2, 2, -2), sprite_position: vec3!(2, 0, 0), view_matrix: Mat4::IDENTITY, projection_matrix: Mat4::IDENTITY, rot_x: 0.0, rot_y: 0.0 };

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
