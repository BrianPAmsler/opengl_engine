use crate::engine::{vectors::Vector3, game_object::component::Component};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3
}

impl Component for Transform {}

impl Transform {
    pub const ZERO: Transform = Transform { position: Vector3::ZERO, rotation: Vector3::ZERO, scale: Vector3::ZERO };
}