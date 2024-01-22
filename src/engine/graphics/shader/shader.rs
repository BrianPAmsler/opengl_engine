use anyhow::Result;
use gl33::AttributeType;

use crate::engine::graphics::{Graphics, shader::FragmentShader};

use super::{ShaderProgram, ShaderProgramBuilder, VertexShader};

pub enum ShaderSource<'a> {
    Vertex(&'a str),
    TesselationControl(&'a str),
    TesselationEvaluation(&'a str),
    Geometry(&'a str),
    Fragment{ source: &'a str, frag_data: Box<[(u32, &'static str)]>},
    Compute(&'a str)
}

pub struct Attribute {
    pub type_: AttributeType,
    pub size: i32,
    pub normalized: bool
}

pub struct ShaderBuilder<'a> {
    sources: Vec<ShaderSource<'a>>,
    attributes: Vec<Attribute>,
    vbo: Option<u32>
}

impl<'a> ShaderBuilder<'a> {
    pub fn new() -> ShaderBuilder<'a> {
        ShaderBuilder { sources: Vec::new(), attributes: Vec::new(), vbo: None }
    }

    pub fn add_source(&mut self, source: ShaderSource<'a>) {
        self.sources.push(source);
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn compile(self, gfx: &Graphics) -> Result<Shader> {
        let mut program = ShaderProgramBuilder::new(gfx);
        
        for source in self.sources {
            match source {
                ShaderSource::Vertex(source) => {
                    let shader = VertexShader::compile_shader(gfx, source)?;
                    program.attach_shader(shader);
                },
                ShaderSource::TesselationControl(_) => todo!(),
                ShaderSource::TesselationEvaluation(_) => todo!(),
                ShaderSource::Geometry(_) => todo!(),
                ShaderSource::Fragment { source, frag_data } => {
                    let mut shader = FragmentShader::compile_shader(gfx, source)?;
                    frag_data.to_vec().into_iter().for_each(|(color, name)| shader.add_frag_data(color, name));

                    program.attach_shader(shader);
                },
                ShaderSource::Compute(_) => todo!(),
            }
        }

        let program = program.finish();
        let mut vao = 0;
        gfx.glGenVertexArray(&mut vao);
        gfx.glBindVertexArray(vao);

        let mut offset = 0;
        let mut stride = 0;
        self.attributes.iter().for_each(|attrib| stride += attrib.size);
        self.attributes.into_iter().enumerate().for_each(|(index, attrib)| {
            gfx.glVertexAttribPointer(index as u32, attrib.size, attrib.type_, attrib.normalized, stride, offset);
            gfx.glEnableVertexAttribArray(index as u32);
            offset += attrib.size as u32;
        });
        
        Ok(Shader { program, vbo: self.vbo.unwrap(), vao })
    }
}

pub struct Shader {
    program: ShaderProgram,
    vbo: u32,
    vao: u32
}