use std::{any::TypeId, cell::{Ref, RefCell, RefMut}, collections::HashSet, ops::{Deref, DerefMut}, rc::Rc};

use crate::engine::{data_structures::{AllocationIndex, VecAllocator}, errors::{ObjectError, Result}, graphics::Graphics, Engine};

use super::{component::{components::Transform, Component}, GameObject};

pub struct World {
    pub(in crate::engine::game_object) root: ObjectID,
    pub(in crate::engine::game_object) objects: VecAllocator<GameObject>,
    pub(in crate::engine::game_object) components: VecAllocator<Rc<RefCell<Box<dyn Component>>>>
}

#[derive(Clone, Copy, Hash)]
pub struct ObjectID {
    idx: AllocationIndex
}

#[derive(Clone, Copy, Hash)]
pub struct ComponentID {
    idx: AllocationIndex,
    type_: TypeId
}

// #[self_referencing]
// pub struct ComponentRef<'a, C: Component> {
//     ref_outer: Ref<'a, VecAllocator<RefCell<Box<dyn Component>>>>,
//     #[borrows(ref_outer)]
//     #[covariant]
//     ref_inner: Ref<'this, C>
// }

// impl<'a, C: Component> Deref for ComponentRef<'a, C> {
//     type Target = C;

//     fn deref(&self) -> &Self::Target {
//         self.borrow_ref_inner().deref()
//     }
// }

pub struct ComponentRefMut<'a, C: Component> {
    ref_outer: Ref<'a, VecAllocator<RefCell<Box<dyn Component>>>>,
    ref_inner: RefMut<'a, C>
}

impl<'a, C: Component> Deref for ComponentRefMut<'a, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.ref_inner.deref()
    }
}

impl<'a, C: Component> DerefMut for ComponentRefMut<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_inner.deref_mut()
    }
}

impl World {
    pub fn new() -> World {
        let mut objects = VecAllocator::new();
        let root = objects.insert(GameObject { name: "root".to_owned(), parent: ObjectID { idx: AllocationIndex::null() }, components: Vec::new(), children: HashSet::new() });
        let root = ObjectID { idx: root };

        let mut world = World {
            root,
            objects,
            components: VecAllocator::new()
        };

        world.add_component(world.root, Transform::ZERO).expect("This also shouldn't happen!");

        world
    }

    pub fn init(&mut self, graphics: &Graphics) -> Result<()> {
        // I really hope the compiler can optimize this nonsense

        let components: Vec<(ObjectID, ComponentID)> = self.objects.iter().flat_map(|(idx, obj)| {
            let owner = ObjectID { idx };

            let children: Vec<_> = obj.components.iter().map(|child| {
                (owner, child.to_owned())
            }).collect();

            children
        }).collect();

        let components: Vec<(ObjectID, Rc<RefCell<Box<dyn Component>>>)> = components.into_iter().map(|(owner, component)| {
            let rc = self.components.get(component.idx).ok_or(ObjectError::DeadComponentError)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().init(graphics, owner).unwrap(); // TODO: Fix error types

            Ok(())
        })
    }

    pub fn update(&mut self, graphics: &Graphics, delta_time: f32) -> Result<()> {
        // I really hope the compiler can optimize this nonsense

        let components: Vec<(ObjectID, ComponentID)> = self.objects.iter().flat_map(|(idx, obj)| {
            let owner = ObjectID { idx };

            let children: Vec<_> = obj.components.iter().map(|child| {
                (owner, child.to_owned())
            }).collect();

            children
        }).collect();

        let components: Vec<(ObjectID, Rc<RefCell<Box<dyn Component>>>)> = components.into_iter().map(|(owner, component)| {
            let rc = self.components.get(component.idx).ok_or(ObjectError::DeadComponentError)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().update(graphics, owner, delta_time).unwrap(); // TODO: Fix error types

            Ok(())
        })
    }

    pub fn fixed_update(&mut self, graphics: &Graphics, delta_time: f32) -> Result<()> {
        // I really hope the compiler can optimize this nonsense

        let components: Vec<(ObjectID, ComponentID)> = self.objects.iter().flat_map(|(idx, obj)| {
            let owner = ObjectID { idx };

            let children: Vec<_> = obj.components.iter().map(|child| {
                (owner, child.to_owned())
            }).collect();

            children
        }).collect();

        let components: Vec<(ObjectID, Rc<RefCell<Box<dyn Component>>>)> = components.into_iter().map(|(owner, component)| {
            let rc = self.components.get(component.idx).ok_or(ObjectError::DeadComponentError)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().fixed_update(graphics, owner, delta_time).unwrap(); // TODO: Fix error types

            Ok(())
        })
    }

    // pub fn get_name(&self, object: ObjectID) -> Result<&str> {

    // }

    pub fn get_root(&self) -> ObjectID {
        self.root
    }

    pub fn add_component<C: Component>(&mut self, object: ObjectID, component: C) -> Result<()> {
        let idx = self.components.insert(Rc::new(RefCell::new(Box::new(component))));

        let object = self.objects.get_mut(object.idx).ok_or(ObjectError::DeadObjectError)?;

        object.components.push(ComponentID { idx, type_: TypeId::of::<C>() });

        Ok(())
    }

    pub fn borrow_component<C: Component>(&self, component: ComponentID) -> Result<Ref<C>> {
        let ref_ = self.components.get(component.idx).ok_or(ObjectError::DeadObjectError)?.borrow();

        let downcast = Ref::filter_map(ref_, |t| {
            t.downcast_ref()
        }).map_err(|_| ObjectError::ComponentDowncastError { type_name: std::any::type_name::<C>().to_owned() })?;

        Ok(downcast)
    }

    pub fn borrow_component_mut<C: Component>(&self, component: ComponentID) -> Result<RefMut<C>> {
        let ref_ = self.components.get(component.idx).ok_or(ObjectError::DeadObjectError)?.borrow_mut();

        let downcast = RefMut::filter_map(ref_, |t| {
            t.downcast_mut()
        }).map_err(|_| ObjectError::ComponentDowncastError { type_name: std::any::type_name::<C>().to_owned() })?;

        Ok(downcast)
    }

    pub fn create_game_object(&mut self, name: String, parent: ObjectID) -> ObjectID {
        let new_obj = GameObject { name, parent, components: Vec::new(), children: HashSet::new() };
        let new_obj = ObjectID { idx: self.objects.insert(new_obj) };

        self.add_component(new_obj, Transform::ZERO).expect("This shouldn't happen!");

        new_obj
    }

    pub fn get_component<C: Component>(&self, object: ObjectID) -> Result<ComponentID> {
        let obj = self.objects.get(object.idx).ok_or(ObjectError::DeadObjectError)?;

        for c in obj.components.iter() {
            if c.type_ == TypeId::of::<C>() {
                return Ok(c.clone());
            }
        }

        Err(ObjectError::ComponentNotFoundError)?
    }

    pub fn get_components<C: Component>(&self, object: ObjectID) -> Result<Box<[ComponentID]>> {
        let obj = self.objects.get(object.idx).ok_or(ObjectError::DeadObjectError)?;

        Ok(obj.components.iter().filter_map(|c| {
            if c.type_ == TypeId::of::<C>() {
                Some(c.to_owned())
            } else {
                None
            }
        }).collect())
    }

    pub fn get_children(&self, object: ObjectID) -> Result<Box<[ObjectID]>> {
        let obj = self.objects.get(object.idx).ok_or(ObjectError::DeadObjectError)?;

        Ok(obj.children.iter().map(|child| child.to_owned()).collect())
    }
}