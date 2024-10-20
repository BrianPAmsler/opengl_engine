mod graphics;
mod gl_wrapper;
mod mesh;
mod shader_program;
mod vertex_buffer;
mod texture;

pub mod sprite_renderer;

pub use graphics::*;
pub use gl_wrapper::*;
pub use mesh::*;
pub use shader_program::*;
pub use vertex_buffer::*;
pub use texture::*;

#[cfg(test)]
pub mod test_lock {
    use std::sync::Mutex;

    pub static LOCK: Mutex<()> = Mutex::new(());
}