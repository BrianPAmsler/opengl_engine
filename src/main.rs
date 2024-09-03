#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use engine::{errors::{Error, Result}, game_object::{component::Component, ObjectID}, graphics::{BufferedMesh, CustomAttribute, CustomAttributeData, Graphics, Mesh, RGBColor, VBOBufferer, Vertex}, Engine};
use engine::graphics::{VertexShader, FragmentShader, ShaderProgram, ShaderProgramBuilder};
use gl33::{GL_TRIANGLES, GL_COLOR_BUFFER_BIT};
use glm::{Vec2, Vec3};
use regex::Regex;

use include_crypt_bytes::include_bytes_obfuscate;

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

#[derive(Clone)]
pub struct Renderer {
    current_vao: u32,
    shader_program: ShaderProgram,
    mesh1: BufferedMesh,
    mesh2: BufferedMesh
}

impl Component for Renderer {
    fn init(&mut self, _graphics: &Graphics, _owner: ObjectID) -> Result<()> {
        self.current_vao = self.mesh2.vao();

        _graphics.glClearColor(0.0, 0.0, 0.0, 1.0);

        _graphics.glUseProgram(self.shader_program.program());

        Ok(())
    }

    fn update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<()> {
        let current_mesh = match (_graphics.get_glfw_time() as f32 / 5.0) as i32 % 2 == 0 {
            true => &self.mesh1,
            false => &self.mesh2,
        };

        if self.current_vao != current_mesh.vao() {
            self.current_vao = current_mesh.vao();
            _graphics.glBindVertexArray(current_mesh.vao());
        }

        _graphics.glClear(GL_COLOR_BUFFER_BIT);
        _graphics.glDrawArrays(GL_TRIANGLES, 0, current_mesh.len() as _);

        Ok(())   
    }
}

// The include_bytes_obfuscate! macro generates non upper case globals and doesn't ignore the warning. wtf???
#[allow(non_upper_case_globals)]
fn start_game() -> Result<()> {
    let mut engine = Engine::new()?;
    engine.create_window("Test Window", 800, 600, engine::WindowMode::Windowed)?;

    let world = engine.get_world();

    let a = world.create_game_object("a".to_owned(), world.get_root())?;
    let _b = world.create_game_object("b".to_owned(), a)?;
    let c = world.create_game_object("c".to_owned(), a)?;
    let _d = world.create_game_object("d".to_owned(), c)?;
    let vertex_data = Box::new([
        Vertex { x: -1.0, y: -1.0, z: 0.0 },
        Vertex { x: -1.0, y: 1.0, z: 0.0 },
        Vertex { x: 0.0, y: 0.0, z: 0.0 },
    
        Vertex { x: 1.0, y: 1.0, z: 0.0 },
        Vertex { x: 1.0, y: -1.0, z: 0.0 },
        Vertex { x: 0.0, y: 0.0, z: 0.0 },
    
        Vertex { x: -0.75, y: 1.0, z: 0.0 },
        Vertex { x: 0.75, y: 1.0, z: 0.0 },
        Vertex { x: 0.0, y: 0.25, z: 0.0 },
    
        Vertex { x: -0.75, y: -1.0, z: 0.0 },
        Vertex { x: 0.75, y: -1.0, z: 0.0 },
        Vertex { x: 0.0, y: -0.25, z: 0.0 },
    ]);

    let color_data_1 = Box::new([
        RGBColor { r: 1.0, g: 0.0, b: 0.0 },
        RGBColor { r: 0.0, g: 1.0, b: 0.0 },
        RGBColor { r: 0.0, g: 0.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 1.0, b: 0.0 },
        RGBColor { r: 0.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 0.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
    ]);

    let color_data_2: Box<[CustomAttribute<f32, 3, true>]> = Box::new([
        CustomAttribute::new([1.0, 1.0, 1.0]),
        CustomAttribute::new([1.0, 1.0, 1.0]),
        CustomAttribute::new([1.0, 1.0, 1.0]),
        
        CustomAttribute::new([1.0, 1.0, 1.0]),
        CustomAttribute::new([1.0, 1.0, 1.0]),
        CustomAttribute::new([1.0, 1.0, 1.0]),
        
        CustomAttribute::new([1.0, 0.0, 0.0]),
        CustomAttribute::new([0.0, 1.0, 0.0]),
        CustomAttribute::new([0.0, 0.0, 1.0]),
        
        CustomAttribute::new([0.0, 1.0, 1.0]),
        CustomAttribute::new([1.0, 0.0, 1.0]),
        CustomAttribute::new([1.0, 1.0, 0.0]),
    ]);

    let mesh1 = Mesh::new("Test Mesh".to_owned(), vertex_data.clone(), Some(color_data_1), None, None, None);
    let mut mesh2 = Mesh::new("Test Mesh 2".to_owned(), vertex_data, None, None, None, None);
    mesh2.add_custom_data(CustomAttributeData::new(color_data_2));

    let gfx = engine.get_graphics()?;
    
    let mut vbo = VBOBufferer::new(gfx);
    let mesh1 = vbo.add_mesh(mesh1);
    let mesh2 = vbo.add_mesh(mesh2);
    vbo.buffer_data(gfx);

    let mesh1 = mesh1.take();
    let mesh2 = mesh2.take();

    let vertex_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/vertex_color.vert").unwrap()).unwrap();
    let fragment_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/vertex_color.frag").unwrap()).unwrap();

    let vert_shader = VertexShader::compile_shader(gfx, &vertex_shader_source)?;
    let frag_shader = FragmentShader::compile_shader(gfx, &fragment_shader_source)?;

    let mut program_builder = ShaderProgramBuilder::new(gfx);
    program_builder.attach_shader(vert_shader);
    program_builder.attach_shader(frag_shader);
    let shader_program = program_builder.finish();

    let renderer = Renderer { current_vao: 0, shader_program, mesh1, mesh2 };

    let world = engine.get_world();

    world.add_component(_d, FPSCounter::default())?;
    world.add_component(a, renderer)?;

    engine.run()?;

    Ok(())
}

fn main() {
    let v4 = vec4!(vec3!(0), 1);

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
