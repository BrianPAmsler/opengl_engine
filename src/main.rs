#![cfg_attr(debug_assertions, allow(dead_code))]

mod engine;

use anyhow::Error;
use engine::component::{Component, Engine};
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
    let root = world.get_root();

    let a = world.create_empty("a", root)?;
    let _b= world.create_empty("b", a)?;
    let c = world.create_empty("c", a)?;
    let d = world.create_empty("d", c)?;

    let comp = TestComponent { value: 5 };
    d.add_component(comp)?;

    println!("Sending update...");
    root.update(&None)?;

    println!("Changing component value...");
    let mut comp = d.get_component::<TestComponent>()?.unwrap();
    comp.borrow_mut().value = 69;
    
    println!("Sending update...");
    root.update(&None)?;

    Ok(())
}

fn main() {
    match start_game() {
        Ok(_) => {},
        Err(err) => { print!("{}", clean_backtrace(&err)); }
    }
}

pub fn clean_backtrace(error: &Error) -> String {
    let str = format!("{}", error.backtrace());

    let mut clean_str = String::new();
    clean_str.reserve(str.len());

    clean_str += &format!("Error: {}\n", error.to_string());
    
    let is_error_line = Regex::new("^ +[0-9]+:").unwrap();
    let in_crate = Regex::new("^ +[0-9]+: opengl_engine::").unwrap();

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
