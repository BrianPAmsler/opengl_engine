use std::{cell::RefCell, rc::Rc};

use gl33::{GL_ARRAY_BUFFER, GL_STATIC_DRAW, GL_FLOAT};

use super::{Graphics, Mesh, Normal, RGBColor, Vertex, UV};

pub struct BufferedMeshHandle {
    mesh: Rc<RefCell<Option<BufferedMesh>>>
}

impl BufferedMeshHandle {
    fn new() -> BufferedMeshHandle {
        BufferedMeshHandle { mesh: Rc::new(RefCell::new(None)) }
    }

    fn clone(&self) -> BufferedMeshHandle {
        BufferedMeshHandle { mesh: self.mesh.clone() }
    }

    pub fn finished(&self) -> bool {
        self.mesh.borrow().is_some()
    }

    pub fn take(self) -> BufferedMesh {
        self.mesh.borrow_mut().take().unwrap()
    }
}

pub struct VBOManager {
    vbo: u32,
    vertex_data: usize,
    color_data: usize,
    uv_data: usize,
    normal_data: usize,
    custom_data: usize,
    meshes: Vec<(Mesh, BufferedMeshHandle)>
}

impl VBOManager {
    pub fn new(gfx: &Graphics) -> VBOManager {
        let mut vbo = 0;
        gfx.glGenBuffer(&mut vbo);

        VBOManager { vbo, vertex_data: 0, color_data: 0, uv_data: 0, normal_data: 0, custom_data: 0, meshes: Vec::new() }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> BufferedMeshHandle {
        self.vertex_data += mesh.vertex_data.len() * std::mem::size_of::<Vertex>();

        if mesh.has_color_data() {
            self.color_data += mesh.color_data.len() * std::mem::size_of::<RGBColor>();
        }

        if mesh.has_uv_data() {
            self.uv_data += mesh.uv_data.len() * std::mem::size_of::<UV>();
        }

        if mesh.has_normal_data() {
            self.normal_data += mesh.normal_data.len() * std::mem::size_of::<Normal>();
        }

        for data in &mesh.custom_data {
            self.custom_data += data.data().len();
        }

        let buff = BufferedMeshHandle::new();
        self.meshes.push((mesh, buff.clone()));

        buff 
    }

    pub fn buffer_data(self, gfx: &Graphics) -> u32 {
        let total_size = self.vertex_data + self.color_data + self.uv_data + self.normal_data + self.custom_data;

        gfx.glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
        gfx.glBufferNull(GL_ARRAY_BUFFER, total_size, GL_STATIC_DRAW);

        let mut vertex_offset = 0;
        let mut color_offset = self.vertex_data;
        let mut uv_offset = color_offset + self.color_data;
        let mut normal_offset = uv_offset + self.uv_data;
        let mut custom_offset = normal_offset + self.normal_data;

        for (mesh, buff) in self.meshes {
            let mut vao = 0;
            gfx.glGenVertexArray(&mut vao);
            gfx.glBindVertexArray(vao);
    
            gfx.glBufferSubData(GL_ARRAY_BUFFER, vertex_offset as isize, &mesh.vertex_data);
    
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

            vertex_offset += std::mem::size_of::<Vertex>() * mesh.vertex_data.len();
    
            let mut index = 1;
            if mesh.has_color_data() {
                gfx.glBufferSubData(GL_ARRAY_BUFFER, color_offset as _, &mesh.color_data);

                // Enable color attribute pointer
                gfx.glVertexAttribPointer(
                    index,
                    3,
                    GL_FLOAT,
                    true,
                    0,
                    color_offset as _,
                );

                gfx.glEnableVertexAttribArray(index);
                index += 1;

                color_offset += std::mem::size_of::<RGBColor>() * mesh.color_data.len();
            }
    
            if mesh.has_uv_data() {
                gfx.glBufferSubData(GL_ARRAY_BUFFER, uv_offset as _, &mesh.uv_data);
    
                // Enable uv attribute pointer
                gfx.glVertexAttribPointer(
                    index,
                    2,
                    GL_FLOAT,
                    false,
                    0,
                    uv_offset as _,
                );

                gfx.glEnableVertexAttribArray(index);
                index += 1;

                uv_offset += std::mem::size_of::<UV>() * mesh.uv_data.len();
            }
    
            if mesh.has_normal_data() {
                gfx.glBufferSubData(GL_ARRAY_BUFFER, normal_offset as _, &mesh.normal_data);
    
                // Enable normal attribute pointer
                gfx.glVertexAttribPointer(
                    index,
                    3,
                    GL_FLOAT,
                    true,
                    0,
                    normal_offset as _,
                );
                
                gfx.glEnableVertexAttribArray(index);
                index += 1;

                normal_offset += std::mem::size_of::<Normal>() * mesh.normal_data.len();
            }
    
            for data in &mesh.custom_data {
                gfx.glBufferSubData(GL_ARRAY_BUFFER, custom_offset as _, data.data());
    
                // Enable normal attribute pointer
                gfx.glVertexAttribPointer(
                    index,
                    data.size() as _,
                    data.type_(),
                    data.normalized(),
                    0,
                    custom_offset as _,
                );

                gfx.glEnableVertexAttribArray(index);
                index += 1;

                custom_offset += data.data().len();
            }
    
            let buffered_mesh = BufferedMesh { len: mesh.len(), name: mesh.name, vbo: self.vbo, vao };

            (*buff.mesh.borrow_mut()) = Some(buffered_mesh);
        }

        self.vbo
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
    pub fn name(&self) -> &str {
        &self.name
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