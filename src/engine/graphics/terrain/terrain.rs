use gl46::{GL_RED, GL_RGB};

use crate::engine::graphics::{Graphics, Texture};

pub struct Terrain {
    height_data: Box<[u8]>,
    color_data: Box<[u8]>,
    width: u32,
    height: u32,
    height_texture: Texture,
    color_texture: Texture
}

impl Terrain {
    pub fn new(gfx: &Graphics, width: u32, height: u32) -> Terrain {
        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        let height_data = vec![0; (width + 1) as usize * (height + 1) as usize].into_boxed_slice();
        let color_data = vec![0; (width * height) as usize * 3].into_boxed_slice();

        Self::from_data(gfx, height_data, color_data, width, height)
    }

    pub fn from_data(gfx :&Graphics, height_data: Box<[u8]>, color_data: Box<[u8]>, width: u32, height: u32) -> Terrain {
        if height_data.len() != ((width + 1) * (height + 1)) as usize {
            panic!("Height data size does not match given dimensions.");
        }

        if color_data.len() != (width * height) as usize * 3 {
            panic!("Color data size does not match given dimensions.");
        }

        let height_texture = Texture::new(gfx, &height_data, width + 1, height + 1, GL_RED, GL_RED);
        let color_texture = Texture::new(gfx, &color_data, width, height, GL_RGB, GL_RGB);

        Terrain { height_data, color_data, width, height, height_texture, color_texture }
    }
}