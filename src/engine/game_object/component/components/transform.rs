use gl_types::vectors::Vec3;

use crate::engine::game_object::component::Component;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3
}

impl Component for Transform {}

impl Transform {
    pub const ZERO: Transform = Transform { position: Vec3::ZERO, rotation: Vec3::ZERO, scale: Vec3::ZERO };
}