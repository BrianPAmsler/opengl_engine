use gl33::{GLenum, GL_ARRAY_BUFFER, GL_FLOAT, GL_STATIC_DRAW};

use super::{GLType, Graphics, Normal, RGBColor, Vertex, UV};

#[derive(Clone)]
pub struct CustomAttributeData {
    data: Box<[u8]>,
    type_: GLenum,
    size: i32,
    normalized: bool
}

impl CustomAttributeData {
    pub fn new<T: GLType>(data: Box<[T]>, normalized: bool) -> CustomAttributeData {
        let type_ = T::gl_type();
        let size = data.len() as i32;

        let mut data: Box<[u8]> = unsafe {std::mem::transmute(data) };

        // This is kinda fucked up, but transmute does not adjust the length of the slice.
        // Internally a slice is just a pointer followed by a usize, so we can transmute 
        // it to a tuple and then multiply the usize by the size of our type to adjust
        // the slice's length to be in bytes
        unsafe {
            let raw_slice: &mut (*const u8, usize) = std::mem::transmute(&mut data);
            raw_slice.1 *= std::mem::size_of::<T>();
        }

        CustomAttributeData { data, type_, size, normalized }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Clone, Default)]
pub struct Mesh {
    name: String,
    vertex_data: Box<[Vertex]>,
    color_data: Box<[RGBColor]>,
    uv_data: Box<[UV]>,
    normal_data: Box<[Normal]>,
    custom_data: Vec<CustomAttributeData>
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

    pub fn len(&self) -> usize {
        self.vertex_data.len()
    }

    pub fn size_of(&self) -> usize {
        std::mem::size_of::<Vertex>() * self.vertex_data.len() +
        std::mem::size_of::<RGBColor>() * self.color_data.len() +
        std::mem::size_of::<UV>() * self.uv_data.len() +
        std::mem::size_of::<Normal>() * self.normal_data.len()
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

    pub fn get_vertex_data(&self) -> &[Vertex] {
        &self.vertex_data
    }

    pub fn get_color_data(&self) -> &[RGBColor] {
        if self.color_data.len() == 0 {
            panic!("No color data!");
        }

        &self.color_data
    }

    pub fn get_uv_data(&self) -> &[UV] {
        if self.color_data.len() == 0 {
            panic!("No uv data!");
        }
        
        &self.uv_data
    }

    pub fn get_normal_data(&self) -> &[Normal] {
        if self.color_data.len() == 0 {
            panic!("No normal data!");
        }
        
        &self.normal_data
    }

    pub fn get_custom_data(&self) -> &[CustomAttributeData] {
        &self.custom_data
    }

    pub fn add_custom_data(&mut self, data: CustomAttributeData) {
        self.custom_data.push(data);
    }
}

#[derive(Clone, Default)]
pub struct BufferedMesh {
    name: String,
    vbo: u32,
    vao: u32,
    len: usize
}

impl BufferedMesh {
    pub fn buffer_mesh(gfx: &Graphics, mesh: &Mesh) -> BufferedMesh {
        let mut vbo = 0;
        gfx.glGenBuffer(&mut vbo);
        gfx.glBindBuffer(GL_ARRAY_BUFFER, vbo);

        let mut vao = 0;
        gfx.glGenVertexArray(&mut vao);
        gfx.glBindVertexArray(vao);

        gfx.glBufferNull(GL_ARRAY_BUFFER, mesh.size_of(), GL_STATIC_DRAW);

        gfx.glBufferSubData(GL_ARRAY_BUFFER, 0, mesh.get_vertex_data());

        // Enable pos attribute pointer
        gfx.glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            false,
            0,
            0,
        );
        gfx.glEnableVertexAttribArray(0);

        let mut index = 1;
        let mut offset = mesh.len() * std::mem::size_of::<Vertex>();
        if mesh.has_color_data() {
            gfx.glBufferSubData(GL_ARRAY_BUFFER, offset as _, mesh.get_color_data());

            // Enable color attribute pointer
            gfx.glVertexAttribPointer(
                index,
                3,
                GL_FLOAT,
                true,
                0,
                offset as _,
            );
            gfx.glEnableVertexAttribArray(index);

            offset += mesh.len() * std::mem::size_of::<RGBColor>();
            index += 1;
        }

        if mesh.has_uv_data() {
            gfx.glBufferSubData(GL_ARRAY_BUFFER, offset as _, mesh.get_uv_data());

            // Enable uv attribute pointer
            gfx.glVertexAttribPointer(
                index,
                2,
                GL_FLOAT,
                false,
                0,
                offset as _,
            );
            gfx.glEnableVertexAttribArray(index);

            offset += mesh.len() * std::mem::size_of::<UV>();
            index += 1;
        }

        if mesh.has_normal_data() {
            gfx.glBufferSubData(GL_ARRAY_BUFFER, offset as _, mesh.get_normal_data());

            // Enable normal attribute pointer
            gfx.glVertexAttribPointer(
                index,
                3,
                GL_FLOAT,
                true,
                0,
                offset as _,
            );
            gfx.glEnableVertexAttribArray(index);

            offset += mesh.len() * std::mem::size_of::<Normal>();
            index += 1;
        }

        for data in &mesh.custom_data {
            gfx.glBufferSubData(GL_ARRAY_BUFFER, offset as _, &data.data);

            // Enable normal attribute pointer
            gfx.glVertexAttribPointer(
                index,
                data.size,
                data.type_,
                data.normalized,
                0,
                offset as _,
            );
            gfx.glEnableVertexAttribArray(index);

            offset += data.data.len();
            index += 1;
        }

        BufferedMesh { name: mesh.name.to_owned(), vbo, vao, len: mesh.len() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn vbo(&self) -> u32 {
        self.vbo
    }

    pub fn vao(&self) -> u32 {
        self.vao
    }
}