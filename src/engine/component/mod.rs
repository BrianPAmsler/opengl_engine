use std::{marker::PhantomData, rc::Rc, cell::{RefCell, Ref, RefMut}};

use anyhow::Error;
use downcast_rs::{Downcast, impl_downcast};

use super::game_object::{GameObject, World};

// TODO: Placeholder until Engine is implemented
pub type Engine = Option<PhantomData<()>>;

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

pub struct ComponentRc<C: Component> {
    rc: Rc<RefCell<dyn Component>>,
    _pd: PhantomData<C>
}

impl<C: Component> ComponentRc<C> {
    pub fn downcast_rc(rc: &Rc<RefCell<dyn Component>>) -> Option<ComponentRc<C>> {
        match rc.borrow().is::<C>() {
            true => Some(ComponentRc { rc: rc.clone(), _pd: PhantomData }),
            false => None
        }
    }

    pub fn borrow(&self) -> Ref<C> {
        let borrow = self.rc.borrow();

        Ref::map(borrow, |r| r.downcast_ref().unwrap())
    }

    pub fn borrow_mut(&mut self) -> RefMut<C> {
        let borrow = self.rc.borrow_mut();

        RefMut::map(borrow, |r| r.downcast_mut().unwrap())
    }

    pub fn take_rc(self) -> Rc<RefCell<dyn Component>> {
        self.rc
    }

    pub fn ptr_eq(a: &ComponentRc<C>, b: &ComponentRc<C>) -> bool {
        Rc::ptr_eq(&a.rc, &b.rc)
    }
}