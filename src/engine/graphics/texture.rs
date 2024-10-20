use gl46::{GL_RGBA, GL_TEXTURE_2D, GL_UNSIGNED_BYTE};

use super::Graphics;

pub struct Texture {
    texture_id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub fn buffer_texture(gfx: &Graphics, texture_data: &[u8], width: u32, height: u32) -> Texture {
        let mut texture_id = 0;
        gfx.glGenTexture(&mut texture_id);

        if texture_data.len() > 0 {
            gfx.glBindTexture(GL_TEXTURE_2D, texture_id);
            gfx.glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, &texture_data);
        }

        Texture { texture_id, width, height }
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}