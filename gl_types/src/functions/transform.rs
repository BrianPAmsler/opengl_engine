#![allow(non_snake_case)]

use crate::{matrices::Mat4, vectors::Vec3};

use super::geometric::{cross, dot, normalize};

pub fn lookAt(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let back = normalize(eye - center);
    let right = normalize(cross(back, up));
    let up = cross(right, back);

    let [rx, ry, rz] = right.0.data.0[0];
    let [fx, fy, fz] = back.0.data.0[0];
    let [ux, uy, uz] = up.0.data.0[0];

    let tx = dot(eye, right);
    let ty = dot(eye, up);
    let tz = dot(eye, back * -1);

    Mat4::_new(
        rx, ry, rz, -tx,
        ux, uy, uz, -ty,
        fx, fy, fz, -tz,
        0.0, 0.0, 0.0, 1.0
    )
}

// pub fn rotate(m: &Mat4, angle: f32, axis: &Vec3) -> Mat4 {

// }