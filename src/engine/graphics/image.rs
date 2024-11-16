use std::path::Path;

use image::ImageReader;

use super::{Graphics, Texture};

pub struct Image {
    data: Box<[u8]>,
    width: u32,
    height: u32
}

impl Image {
    pub fn empty(width: u32, height: u32) -> Image {
        Image { data: vec![0; (width * height * 4) as usize].into_boxed_slice(), width, height }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> crate::Result<Image> {
        let img = ImageReader::open(path)?.decode()?.to_rgba8();
        let (width, height) = (img.width(), img.height());
        let data = img.into_raw().into_boxed_slice();
        if data.len() % 4 != 0 {
            panic!("length not divisible by 4");
        }

        Ok(Image { data, width, height })
    }

    pub fn pixel(&self, x: u32, y: u32) -> &(u8, u8, u8, u8) {
        if x >= self.width {
            panic!("x >= width ({} >= {})", x, self.width);
        }
        if y >= self.height {
            panic!("y >= height ({} >= {})", y, self.height);
        }
        
        // I'm not 100% sure this works in all cases, but it seems to work
        let tuples = (&self.data).as_ptr() as *const (u8, u8, u8, u8);

        unsafe { tuples.wrapping_add((x + y * self.width) as usize).as_ref().unwrap() }
    }

    pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut (u8, u8, u8, u8) {
        if x >= self.width {
            panic!("x >= width ({} >= {})", x, self.width);
        }
        if y >= self.height {
            panic!("y >= height ({} >= {})", y, self.height);
        }
        
        // I'm not 100% sure this works in all cases, but it seems to work
        let tuples = (&mut self.data).as_ptr() as *mut (u8, u8, u8, u8);

        unsafe { tuples.wrapping_add((x + y * self.width) as usize).as_mut().unwrap() }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    pub fn as_texture(self, gfx: &Graphics) -> Texture {
        Texture::new(gfx, &self.data, self.width as u32, self.height as u32)
    }

    pub fn blit(&mut self, src: &Image, x: u32, y: u32) {
        for r in 0..src.height {
            for c in 0..src.width {
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

pub fn pad_sprite(image: &Image, width: u32) -> Image {
    let mut new_img = Image::empty(image.width + width * 2, image.height + width * 2);

    // Place image
    new_img.blit(image, width, width);

    // Fill edges

    // top/bottom
    for i in width..image.width + width {
        let top_color = *new_img.pixel(i, width);
        let bottom_color = *new_img.pixel(i, new_img.height - width - 1);

        for j in 0..width {
            *new_img.pixel_mut(i, j) = top_color;
            *new_img.pixel_mut(i, new_img.height - j - 1) = bottom_color;
        }
    };

    // sides
    for i in 0..new_img.height {
        let left_color = *new_img.pixel(width, i);
        let right_color = *new_img.pixel(new_img.width - width - 1, i);

        for j in 0..width {
            *new_img.pixel_mut(j, i) = left_color;
            *new_img.pixel_mut(new_img.width - j - 1, i) = right_color;
        }
    }

    new_img
}

#[cfg(test)]
mod test {
    use function_name::named;
    use pathbuf::pathbuf;

    use crate::engine::graphics::image::{pad_sprite, Image};

    macro_rules! assert_img {
        ($a:expr, $b:expr) => {
            if $a.data != $b.data {
                let path = pathbuf!("test_files", "output", &(function_name!().to_owned() + "_error.png"));

                let mut outfile = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)?;

                let out = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw($b.width as u32, $b.height as u32, &$b.data[..]).unwrap();

                out.write_to(&mut outfile, image::ImageFormat::Png)?;

                return Err("images do not match")?;
            }
        };
    }

    #[test]
    fn image_test() -> crate::Result<()> {
        let img = Image::load_from_file(pathbuf!("test_files", "input", "test_image.png"))?;

        assert_eq!(img.pixel(645, 213), &(65, 134, 212, 255));
        assert_eq!(img.pixel(764, 844), &(121, 65, 68, 255));

        Ok(())
    }

    #[test]
    #[named]
    fn image_blit_test() -> crate::Result<()> {
        let mut bg = Image::load_from_file(pathbuf!("test_files", "input", "test_image.png"))?;
        let overlay = Image::load_from_file(pathbuf!("test_files", "input", "test_small_image.png"))?;

        let expected = Image::load_from_file(pathbuf!("test_files", "input", "blit_test.png"))?;

        bg.blit(&overlay, 54, 24);

        assert_img!(expected, bg);

        Ok(())
    }

    #[test]
    #[named]
    fn image_pad_test() -> crate::Result<()>{
        let img = Image::load_from_file(pathbuf!("test_files", "input", "test_small_image.png"))?;

        let expected_1 = Image::load_from_file(pathbuf!("test_files", "input", "test_pad_1.png"))?;
        let expected_5 = Image::load_from_file(pathbuf!("test_files", "input", "test_pad_5.png"))?;

        let pad_1 = pad_sprite(&img, 1);
        let pad_5 = pad_sprite(&img, 5);

        assert_img!(expected_1, pad_1);
        assert_img!(expected_5, pad_5);

        Ok(())
    }
}
