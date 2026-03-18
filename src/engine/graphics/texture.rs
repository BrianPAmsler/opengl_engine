use crate::engine::graphics::gl_enums::{PixelFormat, PixelType, TextureTarget};

use super::Graphics;

pub struct Texture {
    texture_id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub unsafe fn update_texture(&self, gfx: &Graphics, texture_data: &[u8], format: PixelFormat) {

        gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, self.texture_id);
        gfx.glTextureSubImage2D(self.texture_id, 0, 0, 0, self.width, self.height, format, PixelType::GL_UNSIGNED_BYTE, texture_data);
        gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, 0);
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

pub mod builder {
    use crate::engine::graphics::{Graphics, Texture, gl_enums::{InternalFormat, PixelFormat, PixelType, TextureMagFilter, TextureMinFilter, TextureParameterName, TextureTarget, TextureWrapMode}};

    pub struct TextureBuilder<'a> {
        data: &'a [u8],
        width: u32,
        height: u32,
        internal_format: InternalFormat,
        format: PixelFormat,
        wrap_s: TextureWrapMode,
        wrap_t: TextureWrapMode,
        min_filter: TextureMinFilter,
        mag_filter: TextureMagFilter,
    }

    impl<'a> TextureBuilder<'a> {
        pub unsafe fn from_raw_pixels_unchecked(data: &[u8], width: u32, height: u32, internal_format: InternalFormat, format: PixelFormat) -> TextureBuilder<'_> {
            TextureBuilder {
                data,
                width,
                height,
                internal_format,
                format,
                wrap_s: TextureWrapMode::GL_REPEAT,
                wrap_t: TextureWrapMode::GL_REPEAT,
                min_filter: TextureMinFilter::GL_LINEAR,
                mag_filter: TextureMagFilter::GL_LINEAR,
            }
        }

        pub fn from_raw_pixels(data: &[u8], width: u32, height: u32, internal_format: InternalFormat, format: PixelFormat) -> TextureBuilder<'_> {
            let _ignore = (data, width, height, internal_format, format);
            let todo = todo!("Cannot be implemented until pixel format enums are properly wrapped.");
            // unsafe { Self::from_raw_data(data, width, height) }
        }

        pub fn wrap_s(mut self, wrap_s: TextureWrapMode) -> Self {
            self.wrap_s = wrap_s;
            self
        }

        pub fn wrap_t(mut self, wrap_t: TextureWrapMode) -> Self {
            self.wrap_t = wrap_t;
            self
        }

        pub fn min_filter(mut self, min_filter: TextureMinFilter) -> Self {
            self.min_filter = min_filter;
            self
        }

        pub fn mag_filter(mut self, mag_filter: TextureMagFilter) -> Self {
            self.mag_filter = mag_filter;
            self
        }

        pub fn finish(self, gfx: &Graphics) -> Texture {
            let Self { data, width, height, internal_format, format, wrap_s, wrap_t, min_filter, mag_filter } = self;

            let mut texture_id = 0;
            gfx.glGenTexture(&mut texture_id);

            if data.len() > 0 {
                gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, texture_id);

                gfx.glTexParameteri(TextureTarget::GL_TEXTURE_2D, TextureParameterName::GL_TEXTURE_WRAP_S, gl46::GLenum(wrap_s as u32));	
                gfx.glTexParameteri( TextureTarget::GL_TEXTURE_2D, TextureParameterName::GL_TEXTURE_WRAP_T, gl46::GLenum(wrap_t as u32));
                gfx.glTexParameteri(TextureTarget::GL_TEXTURE_2D, TextureParameterName::GL_TEXTURE_MIN_FILTER, gl46::GLenum(min_filter as u32));
                gfx.glTexParameteri(TextureTarget::GL_TEXTURE_2D, TextureParameterName::GL_TEXTURE_MAG_FILTER, gl46::GLenum(mag_filter as u32));

                unsafe { gfx.glTexImage2D(TextureTarget::GL_TEXTURE_2D, 0, internal_format, width, height, 0, format, PixelType::GL_UNSIGNED_BYTE, data) };
                // TODO: mipmaps
                // gfx.glGenerateMipmap(GL_TEXTURE_2D);

                gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, 0);
            }

            Texture { texture_id, width, height }
        }
    }
}