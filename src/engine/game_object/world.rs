
use std::{cell::RefCell, ptr, collections::VecDeque};

use crate::engine::vectors::Vector3;

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

    pub(in crate::engine::game_object) fn set_parent(&self, parent: u32, child: u32) {
        let mut temp = self.obj_list.borrow_mut();
        let old_parent = temp[child as usize].as_ref().map_or(0, |t| t.parent);
        
        temp[old_parent as usize].as_mut().unwrap().children.remove(&child);
        temp[parent as usize].as_mut().unwrap().children.insert(child);
        temp[child as usize].as_mut().unwrap().parent = parent;
    }

    pub(in crate::engine::game_object) fn get_all_children(&self, obj: u32) -> Box<[u32]> {
        let mut objects: Vec<u32> = Vec::new();

        let mut q = VecDeque::new();
        q.push_back(obj);
        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            let temp = &self.obj_list.borrow()[current as usize];
            let obj = temp.as_ref().unwrap();

            q.extend(obj.children.iter());
            objects.extend(obj.children.iter());
        }

        objects.into_boxed_slice()
    }

    pub fn get_root(&self) -> GameObject {
        self.id_to_game_object(*self.root.borrow())
    }

    pub fn create_empty(&self, name: &str, parent: GameObject) -> GameObject {
        if !ptr::eq(self, parent.world) {
            panic!("Parent from another world!");
        }

        let id = self.add_object(_GameObject::empty(name));
        self.set_parent(parent.id, id);

        self.id_to_game_object(id)
    }

    pub fn create_at_pos(&self, name: &str, parent: GameObject, position: Vector3) -> GameObject {
        if !ptr::eq(self, parent.world) {
            panic!("Parent from another world!");
        }

        let id = self.add_object(_GameObject::at_pos(name, position));
        self.set_parent(parent.id, id);

        self.id_to_game_object(id)
    }

    pub fn create_object(&self, name: &str, parent: GameObject, position: Vector3, rotation: Vector3, scale: Vector3) -> GameObject {
        if !ptr::eq(self, parent.world) {
            panic!("Parent from another world!");
        }

        let id = self.add_object(_GameObject::new(name, position, rotation, scale));
        self.set_parent(parent.id, id);

        self.id_to_game_object(id)
    }

    pub fn destroy(&self, obj: GameObject) {
        let children = obj.get_children();

        // DFS destroy children
        for child in Vec::from(children).into_iter() {
            self.destroy(child);
        }

        // Remove self from parent's child list
        let mut temp = self.obj_list.borrow_mut();
        let parent = temp[obj.id as usize].as_ref().unwrap().parent as usize;
        let parent = temp[parent].as_mut().unwrap();

        parent.children.remove(&obj.id);

        // De-allocate _GameObject
        temp[obj.id as usize] = None;
    }
}