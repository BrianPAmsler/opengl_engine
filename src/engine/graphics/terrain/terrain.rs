use gl46::{GL_RED, GL_RGB, GL_TEXTURE_2D, GL_TEXTURE0, GL_TEXTURE1};

use crate::engine::graphics::{Graphics, Texture, Vertex};

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    All
}

pub struct TerrainCell<'a> {
    top_left: &'a u8,
    top_right: &'a u8,
    bottom_left: &'a u8,
    bottom_right: &'a u8,
    color: &'a [u8; 3]
}

pub struct TerrainCellMut<'a> {
    top_left: &'a mut u8,
    top_right: &'a mut u8,
    bottom_left: &'a mut u8,
    bottom_right: &'a mut u8,
    color: &'a mut [u8; 3]
}

pub struct Terrain {
    height_data: Box<[u8]>,
    color_data: Box<[u8]>,
    width: u32,
    height: u32,
    height_texture: Texture,
    color_texture: Texture,
    height_outdated: bool,
    color_outdated: bool
}

impl Terrain {
    pub fn new(gfx: &Graphics, width: u32, height: u32) -> Terrain {
        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        let height_data = vec![0; (width + 1) as usize * (height + 1) as usize].into_boxed_slice();
        let color_data = vec![0; (width * height) as usize * 3].into_boxed_slice();

        Self::from_raw(gfx, height_data, color_data, width, height)
    }

    pub fn from_raw(gfx :&Graphics, height_data: Box<[u8]>, color_data: Box<[u8]>, width: u32, height: u32) -> Terrain {
        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        if height_data.len() != ((width + 1) * (height + 1)) as usize {
            panic!("Height data size does not match given dimensions.");
        }

        if color_data.len() != (width * height) as usize * 3 {
            panic!("Color data size does not match given dimensions.");
        }

        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        // The buffer length is checked, so we know this is safe.
        let height_texture = unsafe { Texture::from_raw_pixels(gfx, &height_data, width + 1, height + 1, GL_RED, GL_RED) };
        let color_texture = unsafe { Texture::from_raw_pixels(gfx, &color_data, width, height, GL_RGB, GL_RGB) };

        Terrain { height_data, color_data, width, height, height_texture, color_texture, height_outdated: false, color_outdated: false }
    }

    pub fn get_cell<'a>(&'a self, x: u32, z: u32) -> Option<TerrainCell<'a>> {
        if x > self.width || z > self.height {
            return None;
        }

        
        let i = x + z * self.width;
        let bottom_left = &self.height_data[i as usize];

        let i = i as usize * 3;
        let color = (&self.color_data[i..i + 3]).try_into().unwrap();

        let i = (x + 1) + z * self.width;
        let bottom_right = &self.height_data[i as usize];

        let i = x + (z + 1) * self.width;
        let top_left = &self.height_data[i as usize];

        let i = (x + 1) + (z + 1) * self.width;
        let top_right = &self.height_data[i as usize];

        Some(TerrainCell { top_left, top_right, bottom_left, bottom_right, color })
    }

    pub fn get_cell_mut<'a>(&'a mut self, x: u32, z: u32) -> Option<TerrainCellMut<'a>> {
        if x > self.width || z > self.height {
            return None;
        }

        let i = (x + z * self.width) as usize * 3;
        let color = (&mut self.color_data[i..i + 3]).try_into().unwrap();

        // I wish rust had a smipler way of borrowing multiple elements mutably at the same time
        // It might be worth doing this using unsafe code to make it more readable
        let (a, b) = self.height_data[..].split_at_mut(((z + 1) * self.width) as usize);
        let current_row = &mut a[(z * self.width) as usize..(z * self.width) as usize + self.width as usize];
        let next_row = &mut b[..self.width as usize];

        let (a, b) = next_row.split_at_mut((self.width + 1) as usize);
        let (top_left, top_right) = (&mut a[self.width as usize], &mut b[0]);

        let (a, b) = current_row.split_at_mut((self.width + 1) as usize);
        let (bottom_left, bottom_right) = (&mut a[self.width as usize], &mut b[0]);

        Some(TerrainCellMut { top_left, top_right, bottom_left, bottom_right, color })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(in crate::engine::graphics::terrain) fn update_textures(&mut self, gfx: &Graphics) {
        if self.height_outdated {
            // Terrain enforces correct data buffer size, so this is safe
            unsafe { self.height_texture.update_texture(gfx, &self.height_data, GL_RED) };
            self.height_outdated = false;
        }

        if self.color_outdated {
            // Terrain enforces correct data buffer size, so this is safe
            unsafe { self.color_texture.update_texture(gfx, &self.color_data, GL_RGB) };
            self.color_outdated = false;
        }
    }

    pub(in crate::engine::graphics::terrain) fn bind_textures(&mut self, gfx: &Graphics) {
        gfx.glActiveTexture(GL_TEXTURE0);
        gfx.glBindTexture(GL_TEXTURE_2D, self.height_texture.texture_id());

        gfx.glActiveTexture(GL_TEXTURE1);
        gfx.glBindTexture(GL_TEXTURE_2D, self.color_texture.texture_id());
    }
}