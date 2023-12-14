mod engine;

use std::mem;

use engine::component::Component;

use crate::engine::{game_object::World};

#[derive(Clone, Copy)]
struct TestComponent1 {
    test: i32
}

impl Component for TestComponent1 {}

#[derive(Clone, Copy)]
struct TestComponent2 {
    test: f32
}

impl Component for TestComponent2 {}

fn main() -> anyhow::Result<()> {
    let c1: Box<dyn Component> = Box::new(TestComponent1 { test: 5 });
    let c2: Box<dyn Component> = Box::new(TestComponent2 { test: 5.0 });

    let t = c1.downcast::<TestComponent1>().ok().unwrap();

    println!("vaue: {}", t.test);

    let world = World::new();
    let root = world.get_root();

    let a = world.create_empty("a", root)?;
    let b = world.create_empty("b", a)?;
    let c = world.create_empty("c", a)?;
    let d = world.create_empty("d", c)?;

    let children = root.get_all_children()? ;
    let names: Box<[String]> = children.iter().map(|c| c.get_name().unwrap()).collect();

    println!("children: {:?}", names);

    world.destroy(c)?;

    let children = root.get_all_children()?;
    let names: Box<[String]> = children.iter().map(|c| c.get_name().unwrap()).collect();

    println!("children: {:?}", names);

    let r = d.get_name();
    if r.is_err() {
        let e = r.err().unwrap();
        println!("{}", e.backtrace());
    }

    Ok(())
}
