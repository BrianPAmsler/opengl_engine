#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use anyhow::Error;
use engine::{component::Component, Engine};
use regex::Regex;

use crate::engine::game_object::World;

#[derive(Clone, Copy)]
pub struct TestComponent {
    pub value: i32
}

impl Component for TestComponent {
    fn update(&mut self, _engine: &Engine, _world: &World, _owner: engine::game_object::GameObject) -> Result<(), Error> {
        println!("{}", self.value);

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

    let comp1 = TestComponent { value: 1 };
    let comp2 = TestComponent { value: 2 };
    let comp3 = TestComponent { value: 3 };
    let comp4 = TestComponent { value: 4 };

    a.add_component(comp1)?;
    b.add_component(comp2)?;
    c.add_component(comp3)?;
    d.add_component(comp4)?;

    println!("Sending update...");
    root.update(&Engine {})?;

    println!("Removing comp3...");
    let comp = c.get_component::<TestComponent>()?.unwrap();

    c.remove_component(comp)?;

    world.destroy(d)?;

    root.update(&Engine {})?;

    d.get_name()?;

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
