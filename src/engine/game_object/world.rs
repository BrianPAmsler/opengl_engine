
use std::{cell::RefCell, ptr};
use anyhow::{Result, anyhow, bail};

use crate::engine::errors::ObjectError;

use super::{game_object::_GameObject, component::components::Transform};

pub struct World {
    pub(in crate::engine::game_object) root: RefCell<usize>,
    pub(in crate::engine::game_object) obj_list: RefCell<Vec<Option<Box<_GameObject>>>>, // Change this to a map if this list becomes too big
    pub(in crate::engine::game_object) object_count: RefCell<usize>
}

impl World {
    pub fn new() -> World {
        let world = World {
            root: RefCell::new(0),
            obj_list: RefCell::new(Vec::new()),
            object_count: RefCell::new(0)
        };

        // world.add_object(_GameObject::empty("root"));

        world
    }
}