
use std::{cell::RefCell, ptr};
use anyhow::{Result, anyhow};

use crate::engine::game_object::game_object::DEAD_MESSAGE;

use super::{GameObject, game_object::_GameObject};

pub struct World {
    pub(in crate::engine::game_object) root: RefCell<u32>,
    pub(in crate::engine::game_object) obj_list: RefCell<Vec<Option<Box<_GameObject>>>>, // Change this to a map if this list becomes too big
    pub(in crate::engine::game_object) object_count: RefCell<u32>
}

impl World {
    pub fn new() -> World {
        let world = World {
            root: RefCell::new(0),
            obj_list: RefCell::new(Vec::new()),
            object_count: RefCell::new(0)
        };

        world.add_object(_GameObject::empty("root"));

        world
    }
    
    fn add_object(&self, obj: _GameObject) -> u32 {
        let id = *self.object_count.borrow();
        *self.object_count.borrow_mut() += 1;
        self.obj_list.borrow_mut().push(Some(Box::new(obj)));

        id
    }

    pub(in crate::engine::game_object) fn id_to_game_object<'a>(&'a self, id: u32) -> GameObject<'a> {
        GameObject { id, world: &self }
    }

    pub(in crate::engine::game_object) fn set_parent(&self, parent: u32, child: u32) -> Result<()> {
        let mut temp = self.obj_list.borrow_mut();
        let old_parent = temp[child as usize].as_ref().map_or(0, |t| t.parent);
        
        temp[old_parent as usize].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?.children.remove(&child);
        temp[parent as usize].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?.children.insert(child);
        temp[child as usize].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?.parent = parent;

        Ok(())
    }

    pub fn reserve_objlist(&self, size: usize) {
        self.obj_list.borrow_mut().reserve(size);
    }

    pub fn get_root(&self) -> GameObject {
        self.id_to_game_object(*self.root.borrow())
    }

    pub fn create_empty(&self, name: &str, parent: GameObject) -> Result<GameObject> {
        if !ptr::eq(self, parent.world) {
            return Err(anyhow!("Parent from another world!"));
        }

        let id = self.add_object(_GameObject::empty(name));
        self.set_parent(parent.id, id)?;

        Ok(self.id_to_game_object(id))
    }

    pub fn destroy(&self, obj: GameObject) -> Result<()> {
        let children = obj.get_children()?;

        // DFS destroy children
        for child in Vec::from(children).into_iter() {
            self.destroy(child)?;
        }

        // Remove self from parent's child list
        let mut temp = self.obj_list.borrow_mut();
        let parent = temp[obj.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?.parent as usize;
        let parent = temp[parent].as_mut().ok_or(anyhow!(DEAD_MESSAGE))?;

        parent.children.remove(&obj.id);

        // De-allocate _GameObject
        temp[obj.id as usize] = None;

        Ok(())
    }

    pub fn get_objlist_size(&self) -> usize {
        self.object_count.borrow().to_owned() as usize
    }
}