use gl33::{GL_FRAGMENT_SHADER, GL_COMPILE_STATUS, GL_VERTEX_SHADER};

use crate::engine::graphics::Graphics;
use anyhow::{Result, bail};

use self::private::Seal;

fn compile_shader(gfx: &Graphics, shader: u32) -> Result<()> {
    gfx.glCompileShader(shader);

    let mut status = 0;
    gfx.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

    if status == 0 {
        bail!("Shader compile error\n{}", gfx.glGetShaderInfoLog(shader));
    }

    Ok(())
}

mod private {
    pub trait Seal {}
}

pub trait ShaderTrait: private::Seal {
    fn get_shader(&self) -> u32;
    fn add(&self, _program: u32, _gfx: &Graphics) {}
}

pub struct VertexShader {
    shader: u32
}

impl Seal for VertexShader {}

impl ShaderTrait for VertexShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }
}

impl VertexShader {
    pub fn compile_shader(gfx: &Graphics, source: &str) -> Result<VertexShader> {
        let shader = gfx.glCreateShader(GL_VERTEX_SHADER);

        gfx.glShaderSource(shader, source);
        compile_shader(gfx, shader)?;

        Ok(VertexShader { shader })
    }
}

pub struct FragmentShader {
    shader: u32
}

impl Seal for FragmentShader {}

impl ShaderTrait for FragmentShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }
}

impl FragmentShader {
    pub fn compile_shader(gfx: &Graphics, source: &str) -> Result<FragmentShader> {
        let shader = gfx.glCreateShader(GL_FRAGMENT_SHADER);

        gfx.glShaderSource(shader, source);
        compile_shader(gfx, shader)?;

        Ok(FragmentShader { shader })
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
        self.gfx.glLinkProgram(self.program);
        let shaders = self.shaders.into_iter().map(|s| s.get_shader()).collect();

        ShaderProgram { program: self.program, shaders }
    }
}

#[derive(Clone, Default)]
pub struct ShaderProgram {
    program: u32,
    shaders: Box<[u32]>
}

impl ShaderProgram {
    pub fn program(&self) -> u32 {
        self.program
    }

    pub fn shaders(&self) -> &[u32] {
        &self.shaders
    }
}