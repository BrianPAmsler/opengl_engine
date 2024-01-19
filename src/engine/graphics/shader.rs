use gl33::{GL_FRAGMENT_SHADER, GL_COMPILE_STATUS, GL_VERTEX_SHADER};

use super::Graphics;
use anyhow::{Result, bail};

pub trait ShaderTrait {
    fn get_shader(&self) -> u32;

    fn add(&self, _program: u32, _gfx: &Graphics) {}
}

pub struct VertexShader {
    shader: u32
}

impl ShaderTrait for VertexShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }
}

impl VertexShader {
    pub fn compile_shader(gfx: &Graphics, source: &str) -> Result<VertexShader> {
        let shader = gfx.glCreateShader(GL_VERTEX_SHADER);

        gfx.glShaderSource(shader, source);
        gfx.glCompileShader(shader);

        let mut status = 0;
        gfx.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

        if status == 0 {
            bail!("Vertex shader error:\n{}", gfx.glGetShaderInfoLog(shader));
        }

        Ok(VertexShader { shader })
    }
}

pub struct FragmentShader {
    shader: u32,
    frag_data: Vec<(u32, &'static str)>
}

impl ShaderTrait for FragmentShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }

    fn add(&self, program: u32, gfx: &Graphics) {
        for (color, name) in &self.frag_data {
            gfx.glBindFragDataLocation(program, *color, name);
        }
    }
}

impl FragmentShader {
    pub fn compile_shader(gfx: &Graphics, source: &str) -> Result<FragmentShader> {
        let shader = gfx.glCreateShader(GL_FRAGMENT_SHADER);

        gfx.glShaderSource(shader, source);
        gfx.glCompileShader(shader);

        let mut status = 0;
        gfx.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

        if status == 0 {
            bail!("Fragment shader error:\n{}", gfx.glGetShaderInfoLog(shader));
        }

        Ok(FragmentShader { shader, frag_data: Vec::new() })
    }

    pub fn add_frag_data(&mut self, color: u32, name: &'static str) {
        self.frag_data.push((color, name));
    }
}

pub struct ShaderProgramBuilder<'a> {
    program: u32,
    shaders: Vec<Box<dyn ShaderTrait>>,
    gfx: &'a Graphics
}

impl ShaderProgramBuilder<'_> {
    pub fn new<'a>(gfx: &'a Graphics) -> ShaderProgramBuilder<'a> {
        let program = gfx.glCreateProgram();

        ShaderProgramBuilder { program, shaders: Vec::new(), gfx }
    }

    pub fn attach_shader<S: ShaderTrait + 'static>(&mut self, shader: S) {
        self.gfx.glAttachShader(self.program, shader.get_shader());
        shader.add(self.program, self.gfx);

        self.shaders.push(Box::new(shader));
    }

    pub fn finish(self) -> ShaderProgram {
        ShaderProgram { program: self.program, shaders: self.shaders.into_boxed_slice() }
    }
}

pub struct ShaderProgram {
    program: u32,
    shaders: Box<[Box<dyn ShaderTrait>]>
}

impl ShaderProgram {
    pub fn get_program(&self) -> u32 {
        self.program
    }
}