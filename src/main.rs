#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use anyhow::Error;
use engine::{Engine, game_object::component::{components::Transform, ComponentRef, Component}};
use regex::Regex;

use crate::engine::game_object::World;

#[derive(Clone)]
pub struct TestComponent {
    pub transform: Option<ComponentRef<Transform>>,
    pub value: u32
}

impl Component for TestComponent {
    fn init(&mut self, _engine: &Engine, _world: &World, _owner: engine::game_object::GameObject) -> Result<(), Error> {
        self.transform = _owner.get_component::<Transform>()?;

        Ok(())
    }

    fn update(&mut self, _engine: &Engine, _world: &World, _owner: engine::game_object::GameObject) -> Result<(), Error> {
        self.transform.as_mut().unwrap().borrow_mut()?.position += (0, 1, 0).into();

        Ok(())
    }
}

fn start_game() -> anyhow::Result<()> {
    let world = World::new();
    world.reserve_objlist(10000000);
    let root = world.get_root();

    let a = world.create_empty("a", root)?;
    let b = world.create_empty("b", a)?;
    let c = world.create_empty("c", a)?;
    let d = world.create_empty("d", c)?;

    c.add_component(TestComponent { transform: None, value: 0 })?;

    root.init(&Engine {  })?;

    let comp = c.get_component::<Transform>()?.unwrap();

    println!("Pos: {}", comp.borrow()?.position);

    root.update(&Engine {  })?;

    println!("Pos after update: {}", comp.borrow()?.position);

    Ok(())
}

fn main() {
    match start_game() {
        Ok(_) => {},
        Err(err) => { eprint!("{}", clean_backtrace(&err, "opengl_engine"), ); }
    }
}

pub fn clean_backtrace(error: &Error, crate_name: &'static str) -> String {
    let str = format!("{}", error.backtrace());

    let mut clean_str = String::new();
    clean_str.reserve(str.len());

    clean_str += &format!("Error: {}\n\nStack Backtrace\n", error.to_string());
    
    let is_error_line = Regex::new("^ +[0-9]+:").unwrap();
    let in_crate = Regex::new(&format!("^ +[0-9]+: {}::", crate_name)).unwrap();

    let mut count = 0;
    let mut adding = false;
    for line in str.split('\n') {
        let result = is_error_line.find(line);

        if adding {
            if result.is_some() {
                adding = false;
            } else {
                clean_str += &line;
                clean_str += "\n";
            }
        }
        if !adding {
            match result {
                Some(line_number) => {
                    if in_crate.find(line).is_some() {
                        adding = true;
                        
                        let new_line = format!("   {}: ", count) + &line[line_number.end()..];
                        clean_str += &new_line;
                        clean_str += "\n";
        
                        count += 1;
                    }
                },
                None => {}
            }
        }
    }

    clean_str
}
