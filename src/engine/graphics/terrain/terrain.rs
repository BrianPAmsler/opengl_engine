use crate::engine::graphics::{Graphics, Texture, builder::{TextureBuilder}, gl_enums::{InternalFormat, PixelFormat, TextureMagFilter, TextureMinFilter, TextureTarget, TextureUnit}};

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    All
}

// pub struct TerrainCell<'a> {
//     pub top_left: &'a u8,
//     pub top_right: &'a u8,
//     pub bottom_left: &'a u8,
//     pub bottom_right: &'a u8,
//     pub color: &'a [u8; 3]
// }

pub struct CellCorner<'a> {
    height: &'a mut u8,
    color: &'a mut [u8; 3],
    color_dirty: &'a mut bool,
    height_dirty: &'a mut bool
}

impl CellCorner<'_> {
    pub fn height(&mut self) -> &mut u8 {
        *self.height_dirty = true;
        self.height
    }

    pub fn color(&mut self) -> &mut [u8; 3] {
        *self.color_dirty = true;
        self.color
    }
}

// I'm not the biggest fan of this method of accessing terrain data,
// but I'll keep it for now.
pub struct TerrainCellMut<'a> {
    top_left_height: &'a mut u8,
    top_right_height: &'a mut u8,
    bottom_left_height: &'a mut u8,
    bottom_right_height: &'a mut u8,
    top_left_color: &'a mut [u8; 3],
    top_right_color: &'a mut [u8; 3],
    bottom_left_color: &'a mut [u8; 3],
    bottom_right_color: &'a mut [u8; 3],
    color_changed: &'a mut bool,
    height_changed: &'a mut bool
}

impl<'a> TerrainCellMut<'a> {
    pub fn top_left(&'a mut self) -> CellCorner<'a> {
        CellCorner { height: self.top_left_height, color: self.top_left_color, color_dirty: self.color_changed, height_dirty: self.height_changed }
    }

    pub fn top_right(&'a mut self) -> CellCorner<'a> {
        CellCorner { height: self.top_right_height, color: self.top_right_color, color_dirty: self.color_changed, height_dirty: self.height_changed }
    }

    pub fn bottom_left(&'a mut self) -> CellCorner<'a> {
        CellCorner { height: self.bottom_left_height, color: self.bottom_left_color, color_dirty: self.color_changed, height_dirty: self.height_changed }
    }

    pub fn bottom_right(&'a mut self) -> CellCorner<'a> {
        CellCorner { height: self.bottom_right_height, color: self.bottom_right_color, color_dirty: self.color_changed, height_dirty: self.height_changed }
    }
}

pub struct Terrain {
    height_data: Box<[u8]>,
    color_data: Box<[u8]>,
    width: u32,
    height: u32,
    height_texture: Texture,
    color_texture: Texture,
    height_dirty: bool,
    color_dirty: bool
}

const BYTES_PER_COLOR: usize = 3;
const COLORS_PER_CELL: usize = 4;

impl Terrain {
    pub fn new(gfx: &Graphics, width: u32, height: u32) -> Terrain {
        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        let height_data = vec![0; (width + 1) as usize * (height + 1) as usize].into_boxed_slice();
        let color_data = vec![0; (width * height) as usize * BYTES_PER_COLOR * COLORS_PER_CELL].into_boxed_slice();

        unsafe { Self::from_raw_unchecked(gfx, height_data, color_data, width, height) }
    } 

    pub unsafe fn from_raw_unchecked(gfx :&Graphics, height_data: Box<[u8]>, color_data: Box<[u8]>, width: u32, height: u32) -> Terrain {
        let height_texture = unsafe { TextureBuilder::from_raw_pixels_unchecked(&height_data, width + 1, height + 1, InternalFormat::GL_RED, PixelFormat::GL_RED) }
            .min_filter(TextureMinFilter::GL_NEAREST)
            .mag_filter(TextureMagFilter::GL_NEAREST)
            .finish(gfx);
        let color_texture = unsafe { TextureBuilder::from_raw_pixels_unchecked(&color_data, width * 2, height * 2, InternalFormat::GL_RGB, PixelFormat::GL_RGB) }
            .min_filter(TextureMinFilter::GL_NEAREST)
            .mag_filter(TextureMagFilter::GL_NEAREST)
            .finish(gfx);

        Terrain { height_data, color_data, width, height, height_texture, color_texture, height_dirty: false, color_dirty: false }
    }

    pub fn from_raw(gfx :&Graphics, height_data: Box<[u8]>, color_data: Box<[u8]>, width: u32, height: u32) -> Terrain {
        // Height data is per corner, rather than per cell, so each dimension needs one extra value to represent all corners
        if height_data.len() != ((width + 1) * (height + 1)) as usize {
            panic!("Height data size does not match given dimensions. ({})", height_data.len());
        }

        if color_data.len() != (width * height) as usize * BYTES_PER_COLOR * COLORS_PER_CELL {
            panic!("Color data size does not match given dimensions.");
        }

        unsafe { Self::from_raw_unchecked(gfx, height_data, color_data, width, height) }
    }

    pub fn get_raw_height(&self) -> &[u8] {
        &self.height_data
    }

    pub fn get_raw_colors(&self) -> &[u8] {
        &self.color_data
    }

    // pub fn get_cell<'a>(&'a self, x: u32, z: u32) -> Option<TerrainCell<'a>> {
    //     if x >= self.width || z >= self.height {
    //         return None;
    //     }
    //     let height_data_width = self.width + 1;

    //     let i = (x + z * self.width) as usize * 3;
    //     let color = (&self.color_data[i..i + 3]).try_into().unwrap();

    //     let i = x + z * height_data_width;
    //     let bottom_left = &self.height_data[i as usize];

    //     let i = (x + 1) + z * height_data_width;
    //     let bottom_right = &self.height_data[i as usize];

    //     let i = x + (z + 1) * height_data_width;
    //     let top_left = &self.height_data[i as usize];

    //     let i = (x + 1) + (z + 1) * height_data_width;
    //     let top_right = &self.height_data[i as usize];

    //     Some(TerrainCell { top_left, top_right, bottom_left, bottom_right, color })
    // }

    pub fn get_cell_mut<'a>(&'a mut self, x: u32, z: u32) -> Option<TerrainCellMut<'a>> {
        if x >= self.width || z >= self.height {
            return None;
        }

        // All of these point to different elements of the array, so this should be fine.
        // Using slice.split_at_mut to do the same thing was way too complicated
        unsafe {
            let ptr = (&mut self.color_data[..]).as_mut_ptr();
            let i = (x * 2 + z * self.width * 4) as usize * BYTES_PER_COLOR; // spooky numbers
            let bottom_left_color = (std::slice::from_raw_parts_mut(ptr.add(i), BYTES_PER_COLOR)).try_into().unwrap();
            let bottom_right_color = (std::slice::from_raw_parts_mut(ptr.add(i + BYTES_PER_COLOR), BYTES_PER_COLOR)).try_into().unwrap();
            let top_left_color = (std::slice::from_raw_parts_mut(ptr.add(i + self.width as usize * BYTES_PER_COLOR * 2), BYTES_PER_COLOR)).try_into().unwrap();
            let top_right_color = (std::slice::from_raw_parts_mut(ptr.add(i + self.width as usize * BYTES_PER_COLOR * 2 + BYTES_PER_COLOR), BYTES_PER_COLOR)).try_into().unwrap();

            let height_data_width = self.width + 1;

            let ptr = (&mut self.height_data[..]).as_mut_ptr();
            let i = x + z * height_data_width;
            let bottom_left_height = &mut *(ptr.add(i as usize));

            let i = (x + 1) + z * height_data_width;
            let bottom_right_height = &mut *(ptr.add(i as usize));

            let i = x + (z + 1) * height_data_width;
            let top_left_height = &mut *(ptr.add(i as usize));

            let i = (x + 1) + (z + 1) * height_data_width;
            let top_right_height = &mut *(ptr.add(i as usize));
            
            Some(TerrainCellMut {
                top_left_height,
                top_right_height,
                bottom_left_height,
                bottom_right_height,
                top_left_color,
                top_right_color,
                bottom_left_color,
                bottom_right_color,
                color_changed: &mut self.color_dirty,
                height_changed: &mut self.height_dirty
            })
        }

    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(in crate::engine::graphics::terrain) fn update_textures(&mut self, gfx: &Graphics) {
        if self.height_dirty {
            // Terrain enforces correct data buffer size, so this is safe
            unsafe { self.height_texture.update_texture(gfx, &self.height_data, PixelFormat::GL_RED) };
            self.height_dirty = false;
        }

        if self.color_dirty {
            // Terrain enforces correct data buffer size, so this is safe
            unsafe { self.color_texture.update_texture(gfx, &self.color_data, PixelFormat::GL_RGB) };
            self.color_dirty = false;
        }
    }

    pub(in crate::engine::graphics::terrain) fn bind_textures(&mut self, gfx: &Graphics) {
        gfx.glActiveTexture(TextureUnit::GL_TEXTURE0);
        gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, self.height_texture.texture_id());

        gfx.glActiveTexture(TextureUnit::GL_TEXTURE1);
        gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, self.color_texture.texture_id());
    }
}