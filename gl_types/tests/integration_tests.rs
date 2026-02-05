use gl_types::{functions::geometric::{length, normalize}, mat2, mat3, mat4, matrix::inverse, vec2, vec3, vec4, vectors::{Vec2, Vec3, Vec4, VecN}};
use rand::Rng;

const TEST_COUNT: usize = 100000;

fn rand_array<const N: usize>() -> [f32; N] {
    let mut rng = rand::thread_rng();

    let mut array = [0f32; N];
    for i in 0..N {
        array[i] = rng.gen();
    }

    array
}

fn rand_vec<V: VecN<N>, const N: usize>() -> V {
    V::from_array(rand_array())
}

#[test]
pub fn slices() {
    let mut rng = rand::thread_rng();
    
    for _ in 0..TEST_COUNT {
        // Vec2
        let mut array = [rng.gen(), rng.gen()];
        let mut v = vec2!(array[0], array[1]);
        
        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec2::from_slice(&array);
        let v2 = Vec2::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
        
        // Vec3
        let mut array = [rng.gen(), rng.gen(), rng.gen()];
        let mut v = vec3!(array[0], array[1], array[2]);

        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec3::from_slice(&array);
        let v2 = Vec3::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
        
        // Vec4
        let mut array = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
        let mut v = vec4!(array[0], array[1], array[2], array[3]);

        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec4::from_slice(&array);
        let v2 = Vec4::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
    }
}

#[test]
pub fn constructors() {
    for _ in 0..TEST_COUNT {
        let v2: Vec2 = rand_vec();
        assert_eq!(vec2!(), vec2!(0.0, 0.0));
        assert_eq!(vec2!(v2.x()), vec2!(v2.x(), v2.x()));
        assert_eq!(vec2!(v2.x(), v2.y()), vec2!(v2.x(), v2.y()));
    
        let v3: Vec3 = rand_vec();
        assert_eq!(vec3!(), vec3!(0.0, 0.0, 0.0));
        assert_eq!(vec3!(v3.x()), vec3!(v3.x(), v3.x(), v3.x()));
        assert_eq!(vec3!(vec2!(v3.x()), v3.y()), vec3!(v3.x(), v3.x(), v3.y()));
        assert_eq!(vec3!(v3.x(), vec2!(v3.y())), vec3!(v3.x(), v3.y(), v3.y()));
        assert_eq!(vec3!(v3.x(), v3.y(), v3.z()), vec3!(v3.x(), v3.y(), v3.z()));
    
        let v4: Vec4 = rand_vec();
        assert_eq!(vec4!(), vec4!(0.0, 0.0, 0.0, 0.0));
        assert_eq!(vec4!(v4.x()), vec4!(v4.x(), v4.x(), v4.x(), v4.x()));
        assert_eq!(vec4!(vec3!(v4.x()), v4.y()), vec4!(v4.x(), v4.x(), v4.x(), v4.y()));
        assert_eq!(vec4!(v4.x(), vec3!(v4.y())), vec4!(v4.x(), v4.y(), v4.y(), v4.y()));
        assert_eq!(vec4!(vec2!(v4.x()), vec2!(v4.y())), vec4!(v4.x(), v4.x(), v4.y(), v4.y()));
        assert_eq!(vec4!(vec2!(v4.x()), v4.y(), v4.z()), vec4!(v4.x(), v4.x(), v4.y(), v4.z()));
        assert_eq!(vec4!(v4.x(), vec2!(v4.y()), v4.z()), vec4!(v4.x(), v4.y(), v4.y(), v4.z()));
        assert_eq!(vec4!(v4.x(), v4.y(), vec2!(v4.z())), vec4!(v4.x(), v4.y(), v4.z(), v4.z()));
        assert_eq!(vec4!(v4.x(), v4.y(), v4.z(), v4.w()), vec4!(v4.x(), v4.y(), v4.z(), v4.w()));
    }
}

fn operate_array<const N: usize, F: Fn(f32, f32) -> f32>(a: &[f32; N], b: &[f32; N], f: F) -> [f32; N] {
    let mut out = [0f32; N];
    for i in 0..N {
        out[i] = f(a[i], b[i]);
    }

    out
}

#[test]
pub fn addition() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a + b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) + Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a += b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) + Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a += b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) + Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a += b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn subtraction() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a -  b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) - Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a -= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) - Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a -= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) - Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a -= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn multiplication() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a * b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) * Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a *= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) * Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a *= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) * Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a *= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn division() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a / b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) / Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a /= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) / Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a /= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) / Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a /= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn mat_constructors() {
    let m1 = mat3!(1, 2, 3, 4, 5, 6, 7, 8, 9);
    let m2 = mat4!(m1);

    let expected = mat4!(1, 2, 3, 0, 4, 5, 6, 0, 7, 8, 9, 0, 0, 0, 0, 1);

    assert_eq!(m2, expected);

    let m1 = mat2!(1, 2, 3, 4);
    let m2 = mat4!(m1);

    let expected = mat4!(1, 2, 0, 0, 3, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1);

    assert_eq!(m2, expected);
}

#[test]
pub fn length_test() {
    let v2 = vec2!(3, 4);

    assert_eq!(5.0, length(v2))
}

#[test]
pub fn geom_test() {
    let v = vec3!(2, 0, 0);

    assert_eq!(normalize(v), vec3!(1, 0, 0));
}

#[test]
fn inverse_test() {
    let mat = mat3!(vec3!(1, 4, 7), vec3!(2, 5, 2), vec3!(3, 6, 9));
    let inverse = inverse(mat);

    assert_eq!(inverse, mat3!(vec3!(-11.0 / 12.0, - 1.0 / 6.0, 3.0 / 4.0), vec3!(1.0 / 3.0, 1.0 / 3.0, -1.0 / 3.0), vec3!(1.0 / 12.0, -1.0 / 6.0, 1.0 / 12.0)));
}

// Need more tests, but i'm lazy