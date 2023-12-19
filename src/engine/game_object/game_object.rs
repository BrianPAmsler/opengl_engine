use std::{collections::{HashSet, VecDeque}, ops::{Deref, DerefMut}, cell::RefCell, rc::Rc};

use anyhow::{Result, anyhow, Error};

use crate::engine::{component::{Component, ComponentRc}, Engine};

use super::World;

pub(in crate) const DEAD_MESSAGE: &'static str = "Object has been destroyed!";

#[derive(Clone)]
pub(in crate::engine::game_object) struct _GameObject {
    pub name: String,
    pub parent: u32,
    pub components: Vec<Rc<RefCell<dyn Component>>>,
    pub children: HashSet<u32>
}

impl _GameObject {
    pub fn empty(name: &str) -> _GameObject {
        _GameObject { name: name.to_owned(), parent: 0, components: Vec::new(), children: HashSet::new() }
    }
}

struct GameObjectRef<'a> {
    r: std::cell::Ref<'a, Vec<Option<Box<_GameObject>>>>,
    id: usize
}

impl Deref for GameObjectRef<'_> {
    type Target = _GameObject;

    fn deref(&self) -> &Self::Target {
        self.r[self.id].as_ref().unwrap()
    }
}

struct GameObjectRefMut<'a> {
    r: std::cell::RefMut<'a, Vec<Option<Box<_GameObject>>>>,
    id: usize
}

impl Deref for GameObjectRefMut<'_> {
    type Target = _GameObject;

    fn deref(&self) -> &Self::Target {
        self.r[self.id].as_ref().unwrap()
    }
}

impl DerefMut for GameObjectRefMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.r[self.id].as_mut().unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct GameObject<'a> {
    pub(in crate::engine::game_object) id: u32,
    pub(in crate::engine::game_object) world: &'a World
}

impl<'a> GameObject<'a> {
    fn borrow_game_object(&self) -> Result<GameObjectRef<'a>> {
        let r = self.world.obj_list.borrow();

        r[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(GameObjectRef { r, id: self.id as usize })
    }

    fn borrow_game_object_mut(&self) -> Result<GameObjectRefMut<'a>> {
        let r = self.world.obj_list.borrow_mut();

        r[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(GameObjectRefMut { r, id: self.id as usize })
    }

    pub fn get_name(&self) -> Result<String> {
        let game_object = self.borrow_game_object()?;

        Ok(game_object.name.to_owned())
    }

    pub fn set_name(&self, name: &str) -> Result<()>{
        let mut game_object = self.borrow_game_object_mut()?;

        game_object.name = name.to_owned();

        Ok(())
    }

    pub fn get_parent(&self) -> Result<GameObject> {
        let game_object = self.borrow_game_object()?;

        Ok(self.world.id_to_game_object(game_object.parent))
    }

    pub fn set_parent(&self, parent: GameObject) -> Result<()> {
        if !std::ptr::eq(self.world, parent.world) {
            return Err(anyhow!("Objects must be a part of the same World!"));
        }

        self.world.set_parent(parent.id, self.id)?;

        Ok(())
    }

    pub fn get_children(&self) -> Result<Box<[GameObject]>> {
        let game_object = self.borrow_game_object()?;
        let children = &game_object.children;

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

    pub fn add_component<C: Component>(&self, component: C) -> Result<()>{
        let rc = Rc::new(RefCell::new(component));
        let mut game_object = self.borrow_game_object_mut()?;

        game_object.components.push(rc);

        Ok(())
    }

    pub fn init(&self, _engine: &Engine) -> Result<()> {
        // Init components

        // Grab references to all of the components so we can iterate throuh them without borrowing our _GameObject
        let components: Box<[Rc<RefCell<dyn Component>>]> = self.borrow_game_object()?.components.iter().map(|x| {
            x.clone()
        }).collect();

        // Send init to all components
        components.iter().try_for_each(|c| c.borrow_mut().init(_engine, self.world, self.clone()))?;

        // Send init to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.init(_engine))?;

        Ok(())
    }

    pub fn update(&self, _engine: &Engine) -> Result<()> {
        // Update components

        // Grab references to all of the components so we can iterate throuh them without borrowing our _GameObject
        let components: Box<[Rc<RefCell<dyn Component>>]> = self.borrow_game_object()?.components.iter().map(|x| {
            x.clone()
        }).collect();

        // Send update to all components
        components.iter().try_for_each(|c| c.borrow_mut().update(_engine, self.world, self.clone()))?;

        // Send update to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.update(_engine))?;

        Ok(())
    }

    pub fn fixed_update(&self, _engine: &Engine) -> Result<()> {
        // Update components

        // Grab references to all of the components so we can iterate throuh them without borrowing our _GameObject
        let components: Box<[Rc<RefCell<dyn Component>>]> = self.borrow_game_object()?.components.iter().map(|x| {
            x.clone()
        }).collect();

        // Send update to all components
        components.iter().try_for_each(|c| c.borrow_mut().fixed_update(_engine, self.world, self.clone()))?;

        // Send update to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.fixed_update(_engine))?;

        Ok(())
    }

    pub fn get_component<C: Component>(&self) -> Result<Option<ComponentRc<C>>> {
        let game_object = self.borrow_game_object()?;

        for c in &game_object.components {
            let temp = ComponentRc::downcast_rc(c);

            if temp.is_some() {
                return Ok(temp);
            }
        }

        Ok(None)
    }

    pub fn get_components<C: Component>(&self) -> Result<Box<[ComponentRc<C>]>> {
        let game_object = self.borrow_game_object()?;
        let mut vec = Vec::new();
        vec.reserve(game_object.components.len());

        for c in &game_object.components {
            let temp = ComponentRc::downcast_rc(c);

            if temp.is_some() {
                vec.push(temp.unwrap());
            }
        }

        Ok(vec.into_boxed_slice())
    }

    pub fn get_all_components<C: Component>(&self) -> Result<Box<[ComponentRc<C>]>> {
        let mut vec = Vec::new();

        // Add Self Components
        vec.extend(self.get_components()?.into_vec().into_iter());

        // Add components in children
        self.get_children()?.into_iter().try_for_each(|c| {
            c.get_all_components()?.into_vec().into_iter().for_each(|d| vec.push(d));

            Ok::<(), Error>(())
        })?;

        Ok(vec.into_boxed_slice())
    }

    pub fn remove_component<C: Component>(&self, component: ComponentRc<C>) -> Result<()> {
        let mut game_object = self.borrow_game_object_mut()?;
        let component = component.take_rc();

        game_object.components.retain(|c| !Rc::ptr_eq(c, &component));

        Ok(())
    }
}