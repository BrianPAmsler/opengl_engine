pub mod components;

use std::{marker::PhantomData, cell::{Ref, RefMut}};

use anyhow::{Error, Result, bail};
use downcast_rs::{Downcast, impl_downcast};

use crate::engine::Engine;

use super::{World, GameObject, game_object::GameObjectRef};

pub trait Component: Downcast + CopyCloneRequriement {
    fn init(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) -> Result<(), Error> {Ok(())}
    fn update(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) -> Result<(), Error> {Ok(())}
    fn fixed_update(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) -> Result<(), Error> {Ok(())}
}

impl_downcast!(Component);

pub trait CopyCloneRequriement {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> CopyCloneRequriement for T
where
    T: Component + Clone
{   
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ComponentRef<C: Component> {
    pub(in crate::engine::game_object) world: &'static World,
    pub(in crate::engine::game_object) object_id: usize,
    pub(in crate::engine::game_object) component_index: usize,
    game_object: GameObjectRef<'static>,
    _pd: PhantomData<C>
}

impl<C: Component> ComponentRef<C> {
    pub(in crate::engine::game_object) fn new(game_object: GameObjectRef<'static>, component: &Box<dyn Component>, world: &'static World, object_id: usize, component_index: usize) -> Option<ComponentRef<C>> {
        if !component.is::<C>() {
            return None;
        }

        Some(ComponentRef { world, object_id, component_index, game_object, _pd: PhantomData })
    }

    pub fn borrow(&self) -> Result<Ref<C>> {
        let comp = &self.game_object.components[self.component_index];

        if comp.is_none() {
            bail!("dead component!");
        }

        let bx = comp.as_ref().unwrap().borrow();

        Ok(Ref::map(bx, |t| t.downcast_ref().unwrap()))
    }

    pub fn borrow_mut(&self) -> Result<RefMut<C>> {
        let comp = &self.game_object.components[self.component_index];

        if comp.is_none() {
            bail!("dead component!");
        }

        let bx = comp.as_ref().unwrap().borrow_mut();

        Ok(RefMut::map(bx, |t| t.downcast_mut().unwrap()))
    }

    pub fn ptr_eq(a: &ComponentRef<C>, b: &ComponentRef<C>) -> bool {
        std::ptr::eq(a.world, b.world) && a.object_id == b.object_id && a.component_index == b.component_index
    }
}