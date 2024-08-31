// Wrapper for glm

pub use glm::*;
use private::Sealed;

mod private {
    pub trait Sealed {}
    
    impl Sealed for f32 {}
    impl Sealed for f64 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

pub trait  Number: Sealed {
    fn as_f32(self) -> f32;
}

impl Number for f32 {
    fn as_f32(self) -> f32 {
        self
    }
}

impl Number for f64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
}

impl Number for i32 {
    fn as_f32(self) -> f32 {
        self as f32
    }
}

impl Number for i64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
}

impl Number for u32 {
    fn as_f32(self) -> f32 {
        self as f32
    }
}

impl Number for u64 {
    fn as_f32(self) -> f32 {
        self as f32
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr) => {
        glm::Vec2 { x: $x as f32, $y as f32 }
    };
    ($x:expr, $y:expr) => {
        glm::Vec2 { x: $x as f32, y: $y as f32 }
    };
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr) => {
        glm::Vec3 { x: $x as f32, y: $x as f32, z: $x as f32 }
    };
    ($a:expr, $b:expr) => {{
        trait ToVec3 {
            fn vec3(self) -> glm::Vec3;
        }

        impl<N: crate::engine::glm::Number> ToVec3 for (glm::Vec2, N) {
            fn vec3(self) -> glm::Vec3 {
                let (a, b) = self;
                glm::Vec3 { x: a.x, y: a.y, z: b.as_f32() }
            }
        }

        impl<N: crate::engine::glm::Number> ToVec3 for (N, glm::Vec2) {
            fn vec3(self) -> glm::Vec3 {
                let (a, b) = self;
                glm::Vec3 { x: a.as_f32(), y: b.x, z: b.y }
            }
        }
        
        ($a, $b).vec3()
    }};
    ($x:expr, $y:expr, $z:expr) => {
        glm::Vec3 { x: $x as f32, y: $y as f32, z: $z as f32 }
    };
}

#[macro_export]
macro_rules! vec4 {
    ($x:expr) => {
        glm::Vec4 { x: $x as f32, y: $x as f32, z: $x as f32, w: $x as f32 }
    };
    ($a:expr, $b:expr) => {{
        trait ToVec4 {
            fn vec4(self) -> glm::Vec4;
        }

        impl ToVec4 for (glm::Vec2, glm::Vec2) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b) = self;
                glm::Vec4 { x: a.x, y: a.y, z: b.x, w: b.y }
            }
        }

        impl<N: crate::engine::glm::Number> ToVec4 for (glm::Vec3, N) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b) = self;
                glm::Vec4 { x: a.x, y: a.y, z: a.z, w: b.as_f32() }
            }
        }

        impl<N: crate::engine::glm::Number> ToVec4 for (N, glm::Vec3) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b) = self;
                glm::Vec4 { x: a.as_f32(), y: b.x, z: b.y, w: b.z }
            }
        }

        ($a, $b).vec4()
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        trait ToVec4 {
            fn vec4(self) -> glm::Vec4;
        }
        
        impl<N: crate::engine::glm::Number> ToVec4 for (glm::Vec2, N, N) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b, c) = self;
                glm::Vec4 { x: a.x, y: a.y, z: b.as_f32(), w: c.as_f32() }
            }
        }

        impl<N: crate::engine::glm::Number> ToVec4 for (N, glm::Vec2, N) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b, c) = self;
                glm::Vec4 { x: a.as_f32(), y: b.x, z: b.y, w: c.as_f32() }
            }
        }

        impl<N: crate::engine::glm::Number> ToVec4 for (N, N, glm::Vec2) {
            fn vec4(self) -> glm::Vec4 {
                let (a, b, c) = self;
                glm::Vec4 { x: a.as_f32(), y: b.as_f32(), z: c.x, w: c.y }
            }
        }

        ($a, $b, $c).vec4()
    }};
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        glm::Vec4 { x: $x as f32, y: $y as f32, z: $z as f32, w: $w as f32 }
    };
}
