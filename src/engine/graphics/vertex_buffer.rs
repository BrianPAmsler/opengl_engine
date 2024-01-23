use std::{cell::RefCell, rc::Rc};

use gl33::GL_ARRAY_BUFFER;

use super::{BufferedMesh, Graphics, Mesh, Normal, RGBColor, Vertex, UV};

pub struct MeshID {
    id: usize
}

pub struct BufferingMesh {
    mesh: Rc<RefCell<Option<BufferedMesh>>>
}

impl BufferingMesh {
    fn new() -> BufferingMesh {
        BufferingMesh { mesh: Rc::new(RefCell::new(None)) }
    }

    fn clone(&self) -> BufferingMesh {
        BufferingMesh { mesh: self.mesh.clone() }
    }

    fn finished(&self) -> bool {
        self.mesh.borrow().is_some()
    }

    fn take(self) -> BufferedMesh {
        self.mesh.borrow_mut().take().unwrap()
    }
}

pub struct VBOBuilder {
    vbo: u32,
    vertex_data: usize,
    color_data: usize,
    uv_data: usize,
    normal_data: usize,
    custom_data: usize,
    meshes: Vec<(Mesh, BufferingMesh)>
}

impl VBOBuilder {
    pub fn new(gfx: &Graphics) -> VBOBuilder {
        let mut vbo = 0;
        gfx.glGenBuffer(&mut vbo);

        VBOBuilder { vbo, vertex_data: 0, color_data: 0, uv_data: 0, normal_data: 0, custom_data: 0, meshes: Vec::new() }
    }

    fn count_vertex_data(&mut self, data: &[Vertex]) {
        self.vertex_data += data.len();
    }

    fn count_color_data(&mut self, data: &[RGBColor]) {
        self.color_data += data.len();
    }

    fn count_uv_data(&mut self, data: &[UV]) {
        self.uv_data += data.len();
    }

    fn count_normal_data(&mut self, data: &[Normal]) {
        self.normal_data += data.len();
    }

    fn count_custom_data(&mut self, data: &[u8]) {
        self.custom_data += data.len();
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> BufferingMesh {
        self.count_vertex_data(mesh.get_vertex_data());

        if mesh.has_color_data() {
            self.count_color_data(mesh.get_color_data());
        }

        if mesh.has_uv_data() {
            self.count_uv_data(mesh.get_uv_data());
        }

        if mesh.has_normal_data() {
            self.count_normal_data(mesh.get_normal_data());
        }

        for data in mesh.get_custom_data() {
            self.count_custom_data(data.data());
        }

        let buff = BufferingMesh::new();
        self.meshes.push((mesh, buff.clone()));

        buff
    }

    pub fn buffer_data(self, gfx: &Graphics) -> VBO {
        todo!()
    }
}

pub struct VBO {
    vbo: u32
}