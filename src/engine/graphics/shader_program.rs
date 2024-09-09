use gl33::{GL_FRAGMENT_SHADER, GL_COMPILE_STATUS, GL_VERTEX_SHADER};

use crate::engine::{graphics::Graphics, errors::{Result, GraphicsError}};

use self::private::Seal;

pub struct ShaderSource {
    pub source: String,
    pub filename: String
}

macro_rules! embed_shader_source {
    ($s:literal) => {
        {
            let filename = $s.to_owned();
            #[allow(non_upper_case_globals)]
            let bytes = include_crypt_bytes::include_bytes_obfuscate!($s).unwrap();
            let source = String::from_utf8(bytes).unwrap();

            crate::engine::graphics::ShaderSource { source, filename }
        }
    };
}

pub(crate) use embed_shader_source;

fn compile_shader(gfx: &Graphics, shader: u32, shader_filename: &str) -> Result<()> {
    gfx.glCompileShader(shader);

    let mut status = 0;
    gfx.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

    if status == 0 {
        let error_message = gfx.glGetShaderInfoLog(shader);
        return Err(GraphicsError::ShaderCompileError { src: shader_filename.to_owned(), error_message }.into());
    }

    Ok(())
}

mod private {
    pub trait Seal {}
}

pub trait ShaderTrait: private::Seal {
    fn get_shader(&self) -> u32;
    fn add(&self, _program: u32, _gfx: &Graphics) {}
    fn get_source_filename(&self) -> &str;
}

pub struct VertexShader {
    shader: u32,
    filename: String
}

impl Seal for VertexShader {}

impl ShaderTrait for VertexShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }

    fn get_source_filename(&self) -> &str {
        &self.filename
    }
}

impl VertexShader {
    pub fn compile_shader(gfx: &Graphics, source: ShaderSource) -> Result<VertexShader> {
        let shader = gfx.glCreateShader(GL_VERTEX_SHADER);

        gfx.glShaderSource(shader, &source.source);
        compile_shader(gfx, shader, &source.filename)?;

        Ok(VertexShader { shader, filename: source.filename })
    }
}

pub struct FragmentShader {
    shader: u32,
    filename: String
}

impl Seal for FragmentShader {}

impl ShaderTrait for FragmentShader {
    fn get_shader(&self) -> u32 {
        self.shader 
    }

    fn get_source_filename(&self) -> &str {
        &self.filename
    }
}

impl FragmentShader {
    pub fn compile_shader(gfx: &Graphics, source: ShaderSource) -> Result<FragmentShader> {
        let shader = gfx.glCreateShader(GL_FRAGMENT_SHADER);

        gfx.glShaderSource(shader, &source.source);
        compile_shader(gfx, shader, &source.filename)?;

        Ok(FragmentShader { shader, filename: source.filename })
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

#[derive(Clone)]
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