mod engine;

use crate::engine::game_object::World;

fn main() {
    let world = World::new();
    let root = world.get_root();

    let a = world.create_empty("a", root);
    let b = world.create_empty("b", a);
    let c = world.create_empty("c", a);
    let d = world.create_empty("d", c);

    let children = root.get_all_children();
    let names: Box<[String]> = children.iter().map(|c| c.get_name()).collect();

    println!("children: {:?}", names);

    world.destroy(c);

    let children = root.get_all_children();
    let names: Box<[String]> = children.iter().map(|c| c.get_name()).collect();

    println!("children: {:?}", names);

    d.get_name();
}
