#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use anyhow::{Error, Result};
use engine::{Engine, game_object::{component::Component, GameObject}, graphics::vertex_objects::ColoredVertex};
use gl33::{GL_ARRAY_BUFFER, GL_STATIC_DRAW, GL_VERTEX_SHADER, GL_COMPILE_STATUS, GL_FRAGMENT_SHADER, GL_TRIANGLES, GL_FLOAT, GL_COLOR_BUFFER_BIT};
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

const TEST_TRIANGLE: [ColoredVertex; 3] = [
    ColoredVertex { x: 0.0, y: 1.0, z: 0.0, r: 1.0, g: 0.0, b: 0.0 },
    ColoredVertex { x: -1.0, y: -1.0, z: 0.0, r: 0.0, g: 0.0, b: 1.0 },
    ColoredVertex { x: 1.0, y: -1.0, z: 0.0, r: 0.0, g: 1.0, b: 0.0 },
];

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
    vbo: u32,
    vertex_shader: u32,
    fragment_shader: u32,
    shader_program: u32,
    vao: u32
}

impl Component for Renderer {
    fn init(&mut self, _engine: &Engine, _owner: GameObject) -> Result<(), Error> {
        let gfx = _engine.get_graphics()?;
        gfx.glClearColor(0.0, 0.0, 0.0, 1.0);

        gfx.glGenBuffer(&mut self.vbo);
        gfx.glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
        gfx.glBufferData(GL_ARRAY_BUFFER, &TEST_TRIANGLE, GL_STATIC_DRAW);

        self.vertex_shader = gfx.glCreateShader(GL_VERTEX_SHADER);

        gfx.glShaderSource(self.vertex_shader, VERTEX_SHADER_SOURCE);
        gfx.glCompileShader(self.vertex_shader);

        let mut status = 0;
        gfx.glGetShaderiv(self.vertex_shader, GL_COMPILE_STATUS, &mut status);

        if status == 0 {
            println!("Vertex shader error: {}", gfx.glGetShaderInfoLog(self.vertex_shader));
        }

        self.fragment_shader = gfx.glCreateShader(GL_FRAGMENT_SHADER);

        gfx.glShaderSource(self.fragment_shader, FRAG_SHADER_SOURCE);
        gfx.glCompileShader(self.fragment_shader);

        gfx.glGetShaderiv(self.fragment_shader, GL_COMPILE_STATUS, &mut status);

        if status == 0 {
            println!("Fragment shader error: {}", gfx.glGetShaderInfoLog(self.fragment_shader));
        }

        self.shader_program = gfx.glCreateProgram();
        gfx.glAttachShader(self.shader_program, self.vertex_shader);
        gfx.glAttachShader(self.shader_program, self.fragment_shader);

        gfx.glBindFragDataLocation(self.shader_program, 0, "outColor");

        gfx.glLinkProgram(self.shader_program);
        gfx.glUseProgram(self.shader_program);

        gfx.glGenVertexArray(&mut self.vao);
        gfx.glBindVertexArray(self.vao);

        // Enable pos attribute pointer
        gfx.glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            24,
            0,
        );
        gfx.glEnableVertexAttribArray(0);

        // Enable color attribute pointer
        gfx.glVertexAttribPointer(
            1,
            3,
            GL_FLOAT,
            0,
            24,
            12,
        );
        gfx.glEnableVertexAttribArray(1);

        Ok(())
    }

    fn update(&mut self, _engine: &Engine, _owner: GameObject, _delta_time: f32) -> Result<(), Error> {
        let gfx = _engine.get_graphics()?;
        gfx.glClear(GL_COLOR_BUFFER_BIT);
        gfx.glDrawArrays(GL_TRIANGLES, 0, 3);

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

    _d.add_component(FPSCounter::default())?;
    a.add_component(Renderer::default())?;

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
