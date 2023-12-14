use std::marker::PhantomData;

use downcast_rs::{Downcast, impl_downcast};

use super::game_object::{GameObject, World};

// TODO: Placeholder until Engine is implemented
type Engine = PhantomData<()>;

pub trait Component: Downcast + CopyCloneRequriement {
    fn init(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) {}
    fn update(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) {}
    fn fixed_update(&mut self, _engine: &Engine, _world: &World, _owner: GameObject) {}
}

impl_downcast!(Component);

pub trait CopyCloneRequriement {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> CopyCloneRequriement for T
where
    T: Component + Clone + Copy
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