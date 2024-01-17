pub struct ColoredVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub struct TexturedVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32
}

pub struct TexturedVertexWithNormals {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub normal_z: f32,
    pub u: f32,
    pub v: f32
}