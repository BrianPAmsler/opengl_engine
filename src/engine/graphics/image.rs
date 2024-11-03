use std::{fs::OpenOptions, path::Path};

use image::ImageReader;

use super::{Graphics, Texture};

pub struct Image {
    data: Box<[u8]>,
    width: usize,
    height: usize
}

impl Image {
    pub fn empty(width: usize, height: usize) -> Image {
        Image { data: vec![0; width * height * 4].into_boxed_slice(), width, height }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> crate::Result<Image> {
        let img = ImageReader::open(path)?.decode()?.to_rgba8();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let data = img.into_raw().into_boxed_slice();
        if data.len() % 4 != 0 {
            panic!("length not divisible by 4");
        }

        Ok(Image { data, width, height })
    }

    pub fn pixel(&self, x: usize, y: usize) -> &(u8, u8, u8, u8) {
        if x > self.width {
            panic!("x > width");
        }
        if y > self.height {
            panic!("y > height");
        }
        
        // I'm not 100% sure this works in all cases, but it seems to work
        let tuples = (&self.data).as_ptr() as *const (u8, u8, u8, u8);

        unsafe { tuples.wrapping_add(x + y * self.width).as_ref().unwrap() }
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut (u8, u8, u8, u8) {
        if x > self.width {
            panic!("x > width");
        }
        if y > self.height {
            panic!("y > height");
        }
        
        // I'm not 100% sure this works in all cases, but it seems to work
        let tuples = (&mut self.data).as_ptr() as *mut (u8, u8, u8, u8);

        unsafe { tuples.wrapping_add(x + y * self.width).as_mut().unwrap() }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    pub fn as_texture(self, gfx: &Graphics) -> Texture {
        Texture::new(gfx, &self.data, self.width as u32, self.height as u32)
    }

    pub fn blit(&mut self, src: &Image, x: usize, y: usize) {
        for r in 0..src.width {
            for c in 0..src.height {
                let src_pixel = src.pixel(c, r);

                let new_x = x + c;
                let new_y = y + r;

                if new_x < self.width && new_y < self.height {
                    *(self.pixel_mut(new_x, new_y)) = *src_pixel;
                }
            }
        }
    }
}

#[test]
fn image_test() -> crate::Result<()> {
    let img = Image::load_from_file("test_image.png")?;

    assert_eq!(img.pixel(645, 213), &(65, 134, 212, 255));
    assert_eq!(img.pixel(764, 844), &(121, 65, 68, 255));

    Ok(())
}

#[test]
fn blit_test() -> crate::Result<()> {
    let mut bg = Image::load_from_file("test_image.png")?;
    let overlay = Image::load_from_file("test_small_image.png")?;

    let expected = Image::load_from_file("blit_test.png")?;

    bg.blit(&overlay, 54, 24);

    if &bg.data != &expected.data {
        let mut outfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open("incorrect.png")?;

        let out = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(bg.width as u32, bg.height as u32, &bg.data[..]).unwrap();

        out.write_to(&mut outfile, image::ImageFormat::Png)?;

        return Err("images do not match")?;
    }

    Ok(())
}