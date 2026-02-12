use gl46::{GL_LINEAR, GL_REPEAT, GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, InternalFormat, PixelFormat};

use super::Graphics;

pub struct Texture {
    texture_id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub fn new(gfx: &Graphics, texture_data: &[u8], width: u32, height: u32, internal_format: InternalFormat, format: PixelFormat) -> Texture {
        let mut texture_id = 0;
        gfx.glGenTexture(&mut texture_id);

        println!("texture_id: {}", texture_id);

        if texture_data.len() > 0 {
            gfx.glBindTexture(GL_TEXTURE_2D, texture_id);
            
            gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);	
            gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
            gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

            gfx.glTexImage2D(GL_TEXTURE_2D, 0, internal_format, width, height, 0, format, GL_UNSIGNED_BYTE, texture_data);
            // gfx.glGenerateMipmap(GL_TEXTURE_2D);

            gfx.glBindTexture(GL_TEXTURE_2D, 0);
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