#![allow(non_snake_case)]
use crate::matrices::Mat4;

pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    Mat4::_new(
        2.0 * near / (right - left), 0.0                        , (right + left) / (right - left), 0.0,
        0.0                        , 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0,
        0.0                        , 0.0                        , -(far + near) / (far - near)   , -2.0 * far * near / (far - near),
        0.0                        , 0.0                        , 1.0                           , 0.0
    )
}

pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, zNear: f32, zFar: f32) -> Mat4 {
    Mat4::_new(
        2.0 / (right - left), 0.0                 , 0.0                  , -(right + left) / (right - left),
        0.0                 , 2.0 / (top - bottom), 0.0                  , -(top + bottom) / (top - bottom),
        0.0                 , 0.0                 , -2.0 / (zFar - zNear), -(zFar + zNear) / (zFar - zNear),
        0.0                 , 0.0                 , 0.0                  , 1.0
    )
}

pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let near = -near;
    let far = -far;
    let top = near * (fovy / 2.0); 
    let bottom = -top;
    let right = top * aspect;
    let left = -right;

    frustum(left, right, bottom, top, near, far)
}