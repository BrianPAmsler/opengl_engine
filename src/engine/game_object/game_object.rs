use std::collections::{HashSet, VecDeque};

use anyhow::{Result, anyhow};

use crate::engine::component::Component;

use super::World;

pub(in crate) const DEAD_MESSAGE: &'static str = "Object has been destroyed!";

#[derive(Clone)]
pub(in crate::engine::game_object) struct _GameObject {
    pub name: String,
    pub parent: u32,
    pub components: Vec<Box<dyn Component>>,
    pub children: HashSet<u32>
}

impl _GameObject {
    pub fn empty(name: &str) -> _GameObject {
        _GameObject { name: name.to_owned(), parent: 0, components: Vec::new(), children: HashSet::new() }
    }
}

#[derive(Clone, Copy)]
pub struct GameObject<'a> {
    pub(in crate::engine::game_object) id: u32,
    pub(in crate::engine::game_object) world: &'a World
}

impl<'a> GameObject<'a> {
    pub fn get_name(&self) -> Result<String> {
        let temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(game_object.name.to_owned())
    }

    pub fn set_name(&self, name: &str) -> Result<()>{
        let mut temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?;

        game_object.name = name.to_owned();

        Ok(())
    }

    pub fn get_parent(&self) -> Result<GameObject> {
        let temp = self.world.obj_list.borrow_mut();
        let game_object = temp[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(self.world.id_to_game_object(game_object.parent))
    }

    pub fn set_parent(&self, parent: GameObject) -> Result<()> {
        if !std::ptr::eq(self.world, parent.world) {
            return Err(anyhow!("Objects must be a part of the same World!"));
        }

        self.world.set_parent(parent.id, self.id);

        Ok(())
    }

    pub fn get_children(&self) -> Result<Box<[GameObject]>> {
        let objects = self.world.obj_list.borrow();
        let children = &objects[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?.children;

        Ok(children.iter().map(|c| self.world.id_to_game_object(*c)).collect())
    }

    pub fn get_all_children(&self) -> Result<Box<[GameObject]>> {
        let obj_list = self.world.obj_list.borrow();

        let mut objects: Vec<u32> = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(self.id);
        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            let temp = &obj_list[current as usize];
            let obj = temp.as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

            q.extend(obj.children.iter());
            objects.extend(obj.children.iter());
        }

        Ok(objects.into_iter().map(|id| self.world.id_to_game_object(id)).collect())
    }

    pub fn is_destroyed(&self) -> bool {
        let temp = self.world.obj_list.borrow();
        temp[self.id as usize].is_none()
    }

    pub fn add_component<C: Component>(&self, comonent: C) -> Result<()>{
        let bx = Box::new(comonent);
        let mut temp = self.world.obj_list.borrow_mut();
        let game_object = &temp[self.id as usize].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(())
    }
}