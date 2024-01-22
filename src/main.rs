#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use anyhow::{Error, Result};
use engine::{Engine, game_object::{component::Component, GameObject}, graphics::{shader::{VertexShader, FragmentShader, ShaderProgram, ShaderProgramBuilder}, BufferedMesh, Mesh, RGBColor, Vertex}};
use gl33::{GL_ARRAY_BUFFER, GL_STATIC_DRAW, GL_TRIANGLES, GL_FLOAT, GL_COLOR_BUFFER_BIT};
use regex::Regex;

#[derive(Clone, Default)]
pub struct FPSCounter {
    count: i64,
    fixed_count: i64,
    last_update: f32,
    last_fixed_update: f32
}

impl Component for FPSCounter {
    fn update(&mut self, _engine: &Engine, _owner: GameObject, _delta_time: f32) -> Result<(), Error> {
        self.count += 1;
        let current_tick = _engine.get_time();

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f32 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }

        Ok(())
    }

    fn fixed_update(&mut self, _engine: &Engine, _owner: GameObject, _delta_time: f32) -> Result<(), Error> {
        self.fixed_count += 1;
        let current_tick = _engine.get_time();

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

const VERTEX_SHADER_SOURCE: &'static str = "
#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 vertexColor;

smooth out vec3 color;

void main()
{
    gl_Position = vec4(position, 1.0);
    color = vertexColor;
}";

const FRAG_SHADER_SOURCE: &'static str = "
#version 150 core

in vec3 color;

out vec4 outColor;

void main()
{
    outColor = vec4(color, 1.0);
}";

#[derive(Clone, Default)]
pub struct Renderer {
    current_mesh_vbo: u32,
    shader_program: ShaderProgram,
    mesh1: BufferedMesh,
    mesh2: BufferedMesh
}

impl Component for Renderer {
    fn init(&mut self, _engine: &Engine, _owner: GameObject) -> Result<(), Error> {
        let gfx = _engine.get_graphics()?;

        self.current_mesh_vbo = self.mesh2.vbo();

        gfx.glClearColor(0.0, 0.0, 0.0, 1.0);

        let vert_shader = VertexShader::compile_shader(gfx, VERTEX_SHADER_SOURCE)?;
        let frag_shader = FragmentShader::compile_shader(gfx, FRAG_SHADER_SOURCE)?;

        let mut program_builder = ShaderProgramBuilder::new(gfx);
        program_builder.attach_shader(vert_shader);
        program_builder.attach_shader(frag_shader);
        self.shader_program = program_builder.finish();

        gfx.glUseProgram(self.shader_program.get_program());

        Ok(())
    }

    fn update(&mut self, _engine: &Engine, _owner: GameObject, _delta_time: f32) -> Result<(), Error> {
        let gfx = _engine.get_graphics()?;

        let current_mesh = match (_engine.get_time() / 5.0) as i32 % 2 == 0 {
            true => &self.mesh1,
            false => &self.mesh2,
        };

        if self.current_mesh_vbo != current_mesh.vbo() {
            self.current_mesh_vbo = current_mesh.vbo();
            gfx.glBindBuffer(GL_ARRAY_BUFFER, self.current_mesh_vbo);
            gfx.glBindVertexArray(current_mesh.vao());


        }

        gfx.glClear(GL_COLOR_BUFFER_BIT);
        gfx.glDrawArrays(GL_TRIANGLES, 0, current_mesh.len() as _);

        Ok(())   
    }
}

fn start_game() -> Result<()> {
    let mut engine = Engine::new()?;
    engine.create_window("Test Window", 800, 600, engine::WindowMode::Windowed)?;

    let world = engine.world;

    let a = world.create_empty("a", world.get_root())?;
    let _b = world.create_empty("b", a)?;
    let c = world.create_empty("c", a)?;
    let _d = world.create_empty("d", c)?;
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

    let color_data_2 = Box::new([
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 1.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 0.0, b: 0.0 },
        RGBColor { r: 0.0, g: 1.0, b: 0.0 },
        RGBColor { r: 0.0, g: 0.0, b: 1.0 },
        
        RGBColor { r: 1.0, g: 1.0, b: 0.0 },
        RGBColor { r: 0.0, g: 1.0, b: 1.0 },
        RGBColor { r: 1.0, g: 0.0, b: 1.0 },
    ]);

    let mesh1 = Mesh::new("Test Mesh".to_owned(), vertex_data.clone(), Some(color_data_1), None, None);
    let mesh2 = Mesh::new("Test Mesh 2".to_owned(), vertex_data, Some(color_data_2), None, None);

    let mut renderer = Renderer::default();
    let gfx = engine.get_graphics()?;
    renderer.mesh1 = BufferedMesh::buffer_mesh(gfx, &mesh1);
    renderer.mesh2 = BufferedMesh::buffer_mesh(gfx, &mesh2);

    _d.add_component(FPSCounter::default())?;
    a.add_component(renderer)?;

    engine.run()?;

    Ok(())
}

fn main() {
    match start_game() {
        Ok(_) => {},
        Err(err) => { eprint!("{}", clean_backtrace(&err, "opengl_engine"), ); }
    }
}

pub fn clean_backtrace(error: &Error, crate_name: &'static str) -> String {
    let str = format!("{}", error.backtrace());

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
