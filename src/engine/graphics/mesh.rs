use gl33::GLenum;

use super::{GLType, Normal, RGBColor, Vertex, UV};

#[derive(Clone, Copy)]
pub struct CustomAttribute<T: GLType, const S: usize, const N: bool> {
    #[allow(dead_code)]
    data: [T; S]
}

impl<T: GLType, const S: usize, const N: bool> Default for CustomAttribute<T, S, N>
where
    [T; S]: Default,
{
    fn default() -> Self {
        Self { data: Default::default() }
    }
}

impl<T: GLType, const S: usize, const N: bool> CustomAttribute<T, S, N> {
    pub fn new(data: [T; S]) -> CustomAttribute<T, S, N> {
        CustomAttribute { data }
    }
}

#[derive(Clone)]
pub struct CustomAttributeData {
    data: Box<[u8]>,
    type_: GLenum,
    size: usize,
    len: usize,
    normalized: bool
}

impl CustomAttributeData {
    pub fn new<T: GLType, const S: usize, const N: bool>(data: Box<[CustomAttribute<T, S, N>]>) -> CustomAttributeData {
        let type_ = T::gl_type();
        let size = S;
        let normalized = N;
        let len = data.len();

        let mut data: Box<[u8]> = unsafe {std::mem::transmute(data) };

        // This is kinda fucked up, but transmute does not adjust the length of the slice.
        // Internally a slice is just a pointer followed by a usize, so we can transmute 
        // it to a tuple and then multiply the usize by the size of our type to adjust
        // the slice's length to be in bytes
        unsafe {
            let raw_slice: &mut (*const u8, usize) = std::mem::transmute(&mut data);
            raw_slice.1 *= std::mem::size_of::<T>() * S;
        }

        CustomAttributeData { data, type_, size, normalized, len }
    }

    pub fn from_raw<T: GLType>(data: Box<[T]>, size: usize, normalized: bool) -> CustomAttributeData {
        let type_ = T::gl_type();
        let len = data.len() / size;

        let mut data: Box<[u8]> = unsafe {std::mem::transmute(data) };

        // This is kinda fucked up, but transmute does not adjust the length of the slice.
        // Internally a slice is just a pointer followed by a usize, so we can transmute 
        // it to a tuple and then multiply the usize by the size of our type to adjust
        // the slice's length to be in bytes
        unsafe {
            let raw_slice: &mut (*const u8, usize) = std::mem::transmute(&mut data);
            raw_slice.1 *= std::mem::size_of::<T>();
        }

        CustomAttributeData { data, type_, size, normalized, len }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn type_(&self) -> GLenum {
        self.type_
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn normalized(&self) -> bool {
        self.normalized
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Default)]
pub struct Mesh {
    pub(in crate::engine::graphics) name: String,
    pub(in crate::engine::graphics) vertex_data: Box<[Vertex]>,
    pub(in crate::engine::graphics) color_data: Box<[RGBColor]>,
    pub(in crate::engine::graphics) uv_data: Box<[UV]>,
    pub(in crate::engine::graphics) normal_data: Box<[Normal]>,
    pub(in crate::engine::graphics) custom_data: Vec<CustomAttributeData>
}

impl Mesh {
    pub fn new(name: String, vertex_data: Box<[Vertex]>, color_data: Option<Box<[RGBColor]>>, uv_data: Option<Box<[UV]>>, normal_data: Option<Box<[Normal]>>) -> Mesh {
        let color_data = match color_data {
            Some(data) => {
                if data.len() != vertex_data.len() {
                    panic!("All arrays must be the same length!")
                }

                data
            },
            None => Box::new([])
        };

        let uv_data = match uv_data {
            Some(data) => {
                if data.len() != vertex_data.len() {
                    panic!("All arrays must be the same length!")
                }

                data
            },
            None => Box::new([])
        };

        let normal_data = match normal_data {
            Some(data) => {
                if data.len() != vertex_data.len() {
                    panic!("All arrays must be the same length!")
                }

                data
            },
            None => Box::new([])
        };

        Mesh { name, vertex_data, color_data, uv_data, normal_data, custom_data: Vec::new() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.vertex_data.len()
    }

    pub fn has_color_data(&self) -> bool {
        self.color_data.len() > 0
    }

    pub fn has_uv_data(&self) -> bool {
        self.uv_data.len() > 0
    }

    pub fn has_normal_data(&self) -> bool {
        self.normal_data.len() > 0
    }

    pub fn add_custom_data(&mut self, data: CustomAttributeData) {
        if data.len() != self.vertex_data.len() {
            panic!("Attribute count does not match vertex count!");
        }

        self.custom_data.push(data);
    }
}
