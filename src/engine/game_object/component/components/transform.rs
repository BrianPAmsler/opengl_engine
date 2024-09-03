use crate::engine::glm::Vec3;
use crate::engine::game_object::component::Component;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3
}

impl Component for Transform {}

impl Transform {
    pub const ZERO: Transform = Transform { position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, rotation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 } };
}