use std::collections::{HashSet, VecDeque};

use crate::engine::vectors::Vector3;

use super::World;

const DEAD_MESSAGE: &'static str = "Object has been destroyed!";

#[derive(Clone)]
pub(in crate::engine::game_object) struct _GameObject {
    pub name: String,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub parent: u32,
    pub children: HashSet<u32>
}

impl _GameObject {
    pub fn empty(name: &str) -> _GameObject {
        _GameObject { name: name.to_owned(), position: Vector3:: ZERO, rotation: Vector3::ZERO, scale: Vector3::ZERO, parent: 0, children: HashSet::new() }
    }

    pub fn at_pos(name: &str, position: Vector3) -> _GameObject {
        _GameObject { name: name.to_owned(), position, rotation: Vector3::ZERO, scale: Vector3::ZERO, parent: 0, children: HashSet::new() }
    }

    pub fn new(name: &str, position: Vector3, rotation: Vector3, scale: Vector3) -> _GameObject{
        _GameObject { name: name.to_owned(), position, rotation, scale, parent: 0, children: HashSet::new() }
    }
}

#[derive(Clone, Copy)]
pub struct GameObject<'a> {
    pub(in crate::engine::game_object) id: u32,
    pub(in crate::engine::game_object) world: &'a World
}

impl<'a> GameObject<'a> {
    pub fn get_position(&self) -> Vector3 {
        let temp = self.world.obj_list.borrow();
        let game_object = temp[self.id as usize].as_ref().expect(DEAD_MESSAGE).as_ref();

        game_object.position
    }

    pub fn set_position(&self, position: Vector3) {
        let mut temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_mut().expect(DEAD_MESSAGE).as_mut();

        game_object.position = position;
    }

    pub fn get_name(&self) -> String {
        let temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_ref().expect(DEAD_MESSAGE).as_ref();

        game_object.name.to_owned()
    }

    pub fn set_name(&self, name: &str) {
        let mut temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_mut().expect(DEAD_MESSAGE).as_mut();

        game_object.name = name.to_owned();
    }

    pub fn get_parent(&self) -> GameObject {
        let temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_ref().expect(DEAD_MESSAGE).as_ref();

        self.world.id_to_game_object(game_object.parent)
    }

    pub fn set_parent(&self, parent: GameObject) {
        if !std::ptr::eq(self.world, parent.world) {
            panic!("Objects must be a part of the same World!");
        }

        self.world.set_parent(parent.id, self.id);
    }

    pub fn get_children(&self) -> Box<[GameObject]> {
        let objects = self.world.obj_list.borrow();
        let children = &objects[self.id as usize].as_ref().expect(DEAD_MESSAGE).children;

        children.iter().map(|c| self.world.id_to_game_object(*c)).collect()
    }

    pub fn get_all_children(&self) -> Box<[GameObject]> {
        self.world.get_all_children(self.id).iter().map(|id| self.world.id_to_game_object(*id)).collect()
    }

    pub fn is_destroyed(&self) -> bool {
        let temp = self.world.obj_list.borrow();
        temp[self.id as usize].is_none()
    }
}