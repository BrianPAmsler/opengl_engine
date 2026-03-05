use gl46::{GL_CLAMP_TO_EDGE, GL_LINEAR, GL_NEAREST, GL_REPEAT, GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, InternalFormat, PixelFormat};

use super::Graphics;

pub struct Texture {
    texture_id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub unsafe fn update_texture(&self, gfx: &Graphics, texture_data: &[u8], format: PixelFormat) {

        gfx.glBindTexture(GL_TEXTURE_2D, self.texture_id);
        gfx.glTextureSubImage2D(self.texture_id, 0, 0, 0, self.width, self.height, format, GL_UNSIGNED_BYTE, texture_data);
        gfx.glBindTexture(GL_TEXTURE_2D, 0);
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
    use gl46::{GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, InternalFormat, PixelFormat};

    use crate::engine::graphics::{Graphics, Texture};

    #[repr(u32)]
    pub enum GLTextureMinFilter {
        Nearest = gl46::GL_NEAREST.0,
        Linear = gl46::GL_LINEAR.0,
        NearestMipmapNearest = gl46::GL_NEAREST_MIPMAP_NEAREST.0,
        LinearMipmapNearest = gl46::GL_LINEAR_MIPMAP_NEAREST.0,
        NearestMipmapLinear = gl46::GL_NEAREST_MIPMAP_LINEAR.0,
        LinearMipmapLinear = gl46::GL_LINEAR_MIPMAP_LINEAR.0
    }

    #[repr(u32)]
    pub enum GLTextureMagFilter {
        Nearest = gl46::GL_NEAREST.0,
        Linear = gl46::GL_LINEAR.0
    }

    #[repr(u32)]
    pub enum GLTextureWrap {
        ClampToEdge = gl46::GL_CLAMP_TO_EDGE.0,
        ClampToBorder = gl46::GL_CLAMP_TO_BORDER.0,
        MirroredRepeat = gl46::GL_MIRRORED_REPEAT.0,
        Repeat = gl46::GL_REPEAT.0, 
        MirrorClampToEdge = gl46::GL_MIRROR_CLAMP_TO_EDGE.0
    }

    pub struct TextureBuilder<'a> {
        data: &'a [u8],
        width: u32,
        height: u32,
        internal_format: InternalFormat,
        format: PixelFormat,
        wrap_s: GLTextureWrap,
        wrap_t: GLTextureWrap,
        min_filter: GLTextureMinFilter,
        mag_filter: GLTextureMagFilter,
    }

    impl<'a> TextureBuilder<'a> {
        pub unsafe fn from_raw_pixels_unchecked(data: &'a [u8], width: u32, height: u32, internal_format: InternalFormat, format: PixelFormat) -> TextureBuilder {
            TextureBuilder {
                data,
                width,
                height,
                internal_format,
                format,
                wrap_s: GLTextureWrap::Repeat,
                wrap_t: GLTextureWrap::Repeat,
                min_filter: GLTextureMinFilter::Linear,
                mag_filter: GLTextureMagFilter::Linear,
            }
        }

        pub fn from_raw_pixels(data: &'a [u8], width: u32, height: u32, internal_format: InternalFormat, format: PixelFormat) -> TextureBuilder {
            let _ignore = (data, width, height, internal_format, format);
            let todo = todo!("Cannot be implemented until pixel format enums are properly wrapped.");
            // unsafe { Self::from_raw_data(data, width, height) }
        }

        pub fn wrap_s(mut self, wrap_s: GLTextureWrap) -> Self {
            self.wrap_s = wrap_s;
            self
        }

        pub fn wrap_t(mut self, wrap_t: GLTextureWrap) -> Self {
            self.wrap_t = wrap_t;
            self
        }

        pub fn min_filter(mut self, min_filter: GLTextureMinFilter) -> Self {
            self.min_filter = min_filter;
            self
        }

        pub fn mag_filter(mut self, mag_filter: GLTextureMagFilter) -> Self {
            self.mag_filter = mag_filter;
            self
        }

        pub fn finish(self, gfx: &Graphics) -> Texture {
            let Self { data, width, height, internal_format, format, wrap_s, wrap_t, min_filter, mag_filter } = self;

            let mut texture_id = 0;
            gfx.glGenTexture(&mut texture_id);

            if data.len() > 0 {
                gfx.glBindTexture(GL_TEXTURE_2D, texture_id);

                gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, gl46::GLenum(wrap_s as u32));	
                gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, gl46::GLenum(wrap_t as u32));
                gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, gl46::GLenum(min_filter as u32));
                gfx.glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, gl46::GLenum(mag_filter as u32));

                unsafe { gfx.glTexImage2D(GL_TEXTURE_2D, 0, internal_format, width, height, 0, format, GL_UNSIGNED_BYTE, data) };
                // TODO: mipmaps
                // gfx.glGenerateMipmap(GL_TEXTURE_2D);

                gfx.glBindTexture(GL_TEXTURE_2D, 0);
            }

            Texture { texture_id, width, height }
        }
    }
}