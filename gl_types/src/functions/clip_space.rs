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

pub fn ortho(width: f32, height: f32, zNear: f32, zFar: f32) -> Mat4 {
    let right = width / 2.0;
    let left = -right;
    let top = height / 2.0;
    let bottom = -top;

    Mat4::_new(
        2.0 / (right - left), 0.0                 , 0.0                  , -(right + left) / (right - left),
        0.0                 , 2.0 / (top - bottom), 0.0                  , -(top + bottom) / (top - bottom),
        0.0                 , 0.0                 , 2.0 / (zFar - zNear), -(zFar + zNear) / (zFar - zNear),
        0.0                 , 0.0                 , 0.0                  , 1.0
    )
}

pub fn ortho_aspect(width: f32, aspect: f32, zNear: f32, zFar: f32) -> Mat4 {
    ortho(width, width / aspect, zNear, zFar)
}

pub fn perspective(fovx: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let fovy = 2.0 * f32::atan(f32::tan(fovx / 2.0) / aspect); // Google gemnini formula (i'm trusting that its correct)
    let near = -near;
    let far = -far;
    let top = near * (fovy / 2.0); 
    let bottom = -top;
    let right = top * aspect;
    let left = -right;

    frustum(left, right, bottom, top, near, far)
}