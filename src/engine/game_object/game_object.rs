use std::{collections::{HashSet, VecDeque}, ops::{Deref, DerefMut}, cell::{RefCell, Ref}};

use anyhow::{Result, anyhow, Error, bail};

use crate::engine::Engine;

use super::{World, component::{Component, ComponentRef}};

pub(in crate) const DEAD_MESSAGE: &'static str = "Object has been destroyed!";

#[derive(Clone)]
pub(in crate::engine::game_object) struct _GameObject {
    pub name: String,
    pub parent: usize,
    pub components: Vec<Option<RefCell<Box<dyn Component>>>>,
    pub children: HashSet<usize>
}

impl _GameObject {
    pub fn empty(name: &str) -> _GameObject {
        _GameObject { name: name.to_owned(), parent: 0, components: Vec::new(), children: HashSet::new() }
    }
}

pub(in crate::engine::game_object) struct GameObjectRef<'a> {
    r: std::cell::Ref<'a, Vec<Option<Box<_GameObject>>>>,
    id: usize
}

impl Clone for GameObjectRef<'_> {
    fn clone(&self) -> Self {
        Self { r: Ref::clone(&self.r), id: self.id.clone() }
    }
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
pub struct GameObject {
    pub(in crate::engine::game_object) id: usize,
    pub(in crate::engine::game_object) world: &'static World
}

impl GameObject {
    fn borrow_game_object(&self) -> Result<GameObjectRef<'static>> {
        let r = self.world.obj_list.borrow();

        r[self.id as usize].as_ref().ok_or(anyhow!(DEAD_MESSAGE))?;

        Ok(GameObjectRef { r, id: self.id as usize })
    }

    fn borrow_game_object_mut(&self) -> Result<GameObjectRefMut> {
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

        let mut objects: Vec<usize> = Vec::new();
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
        let rf = RefCell::new(Box::new(component));
        let mut game_object = self.borrow_game_object_mut()?;

        game_object.components.push(Some(rf));

        Ok(())
    }

    pub fn init(&self, _engine: &Engine) -> Result<()> {
        // Init components

        // Send init to all components
        let game_object = self.borrow_game_object()?;
        game_object.components.iter()
            .filter_map(|option| option.as_ref())
            .try_for_each(|c| c.borrow_mut().init(_engine, self.world, self.clone()))?;

        // Send init to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.init(_engine))?;

        Ok(())
    }

    pub fn update(&self, _engine: &Engine) -> Result<()> {
        // Update components

        // Send update to all components
        let game_object = self.borrow_game_object()?;
        game_object.components.iter()
            .filter_map(|option| option.as_ref())
            .try_for_each(|c| c.borrow_mut().update(_engine, self.world, self.clone()))?;

        // Send update to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.update(_engine))?;

        Ok(())
    }

    pub fn fixed_update(&self, _engine: &Engine) -> Result<()> {
        // Update components

        // Send fixed_update to all components
        let game_object = self.borrow_game_object()?;
        game_object.components.iter()
            .filter_map(|option| option.as_ref())
            .try_for_each(|c| c.borrow_mut().fixed_update(_engine, self.world, self.clone()))?;

        // Send update to all children
        let children: Box<[GameObject]> = self.borrow_game_object()?.children.iter().map(|id| {
            self.world.id_to_game_object(*id)
        }).collect();

        children.iter().try_for_each(|c| c.fixed_update(_engine))?;

        Ok(())
    }

    pub fn get_component<C: Component>(&self) -> Result<Option<ComponentRef<C>>> {
        let game_object = self.borrow_game_object()?;

        for c in game_object.components.iter().enumerate().filter_map(|x| x.1.as_ref().map(|t| (x.0, t))) {
            let temp = ComponentRef::new(game_object.clone(), c.1.borrow().deref(), self.world, self.id, c.0);

            if temp.is_some() {
                return Ok(temp);
            }
        }

        Ok(None)
    }

    pub fn get_components<C: Component>(&self) -> Result<Box<[ComponentRef<C>]>> {
        let game_object = self.borrow_game_object()?;
        let mut vec = Vec::new();
        vec.reserve(game_object.components.len());

        for c in game_object.components.iter().enumerate().filter_map(|x| x.1.as_ref().map(|t| (x.0, t))) {
            let temp = ComponentRef::new(self.borrow_game_object()?, c.1.borrow().deref(), self.world, self.id, c.0);

            if temp.is_some() {
                vec.push(temp.unwrap());
            }
        }

        Ok(vec.into_boxed_slice())
    }

    pub fn get_all_components<C: Component>(&self) -> Result<Box<[ComponentRef<C>]>> {
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

    pub fn remove_component<C: Component>(&self, component: ComponentRef<C>) -> Result<()> {
        let mut game_object = self.borrow_game_object_mut()?;
        
        if !std::ptr::eq(self.world, component.world) || component.object_id != self.id {
            bail!("invalid!");
        }

        if game_object.components[component.component_index].is_none() {
            bail!("component dead!")
        }

        game_object.components[component.component_index] = None;

        Ok(())
    }
}