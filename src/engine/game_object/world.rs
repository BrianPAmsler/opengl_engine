use std::{any::TypeId, cell::{Ref, RefCell, RefMut}, collections::HashSet, rc::Rc};

use crate::engine::{data_structures::{AllocationIndex, VecAllocator}, errors::{ObjectError, Result}, graphics::Graphics, input::Input};

use super::{component::{components::Transform, Component}, game_object::GameObject};

pub struct World {
    pub(in crate::engine::game_object) root: ObjectID,
    pub(in crate::engine::game_object) objects: VecAllocator<GameObject>,
    pub(in crate::engine::game_object) components: VecAllocator<Rc<RefCell<Box<dyn Component>>>>
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ObjectID {
    idx: AllocationIndex
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ComponentID {
    idx: AllocationIndex,
    type_: TypeId
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
            let rc = self.components.get(component.idx).map_err(comp_error)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().init(graphics, owner)?; 

            Ok(())
        })
    }

    pub fn update(&mut self, graphics: &Graphics, delta_time: f32, input: &Input) -> Result<()> {
        // I really hope the compiler can optimize this nonsense

        let components: Vec<(ObjectID, ComponentID)> = self.objects.iter().flat_map(|(idx, obj)| {
            let owner = ObjectID { idx };

            let children: Vec<_> = obj.components.iter().map(|child| {
                (owner, child.to_owned())
            }).collect();

            children
        }).collect();

        let components: Vec<(ObjectID, Rc<RefCell<Box<dyn Component>>>)> = components.into_iter().map(|(owner, component)| {
            let rc = self.components.get(component.idx).map_err(comp_error)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().update(graphics, owner, delta_time, input)?; 

            Ok(())
        })
    }

    pub fn fixed_update(&mut self, graphics: &Graphics, delta_time: f32, input: &Input) -> Result<()> {
        // I really hope the compiler can optimize this nonsense

        let components: Vec<(ObjectID, ComponentID)> = self.objects.iter().flat_map(|(idx, obj)| {
            let owner = ObjectID { idx };

            let children: Vec<_> = obj.components.iter().map(|child| {
                (owner, child.to_owned())
            }).collect();

            children
        }).collect();

        let components: Vec<(ObjectID, Rc<RefCell<Box<dyn Component>>>)> = components.into_iter().map(|(owner, component)| {
            let rc = self.components.get(component.idx).map_err(comp_error)?;

            Ok::<(ObjectID, Rc<RefCell<Box<dyn Component>>>), ObjectError>((owner, rc.clone()))
        }).collect::<std::result::Result<Vec<_>, ObjectError>>()?;

        components.into_iter().try_for_each(|(owner, rc)| {
            rc.borrow_mut().fixed_update(graphics, owner, delta_time, input)?; 

            Ok(())
        })
    }

    pub fn get_name(&self, object: ObjectID) -> Result<&str> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        Ok(&obj.name)
    }

    pub fn set_name(&mut self, object: ObjectID, name: String) -> Result<()> {
        let obj = self.objects.get_mut(object.idx).map_err(obj_error)?;

        obj.name = name;

        Ok(())
    }

    pub fn get_root(&self) -> ObjectID {
        self.root
    }

    pub fn add_component<C: Component>(&mut self, object: ObjectID, component: C) -> Result<()> {
        let idx = self.components.insert(Rc::new(RefCell::new(Box::new(component))));

        let object = self.objects.get_mut(object.idx).map_err(obj_error)?;

        object.components.push(ComponentID { idx, type_: TypeId::of::<C>() });

        Ok(())
    }

    pub fn borrow_component<C: Component>(&self, component: ComponentID) -> Result<Ref<C>> {
        let ref_ = self.components.get(component.idx).map_err(obj_error)?.borrow();

        let downcast = Ref::filter_map(ref_, |t| {
            t.downcast_ref()
        }).map_err(|_| ObjectError::ComponentDowncastError { type_name: std::any::type_name::<C>().to_owned() })?;

        Ok(downcast)
    }

    pub fn borrow_component_mut<C: Component>(&self, component: ComponentID) -> Result<RefMut<C>> {
        let ref_ = self.components.get(component.idx).map_err(obj_error)?.borrow_mut();

        let downcast = RefMut::filter_map(ref_, |t| {
            t.downcast_mut()
        }).map_err(|_| ObjectError::ComponentDowncastError { type_name: std::any::type_name::<C>().to_owned() })?;

        Ok(downcast)
    }

    pub fn create_game_object(&mut self, name: String, parent: ObjectID) -> Result<ObjectID> {
        self.objects.get(parent.idx).map_err(obj_error)?;

        let new_obj = GameObject { name, parent: self.root, components: Vec::new(), children: HashSet::new() };
        let new_obj = ObjectID { idx: self.objects.insert(new_obj) };

        self.add_component(new_obj, Transform::ZERO).expect("This shouldn't happen!");
        self.set_parent(new_obj, parent).unwrap();

        Ok(new_obj)
    }

    pub fn get_component<C: Component>(&self, object: ObjectID) -> Result<ComponentID> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        for c in obj.components.iter() {
            if c.type_ == TypeId::of::<C>() {
                return Ok(c.clone());
            }
        }

        Err(ObjectError::ComponentNotFoundError)?
    }

    pub fn get_components<C: Component>(&self, object: ObjectID) -> Result<Box<[ComponentID]>> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        Ok(obj.components.iter().filter_map(|c| {
            if c.type_ == TypeId::of::<C>() {
                Some(c.to_owned())
            } else {
                None
            }
        }).collect())
    }

    pub fn get_children(&self, object: ObjectID) -> Result<Box<[ObjectID]>> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        Ok(obj.children.iter().map(|child| child.to_owned()).collect())
    }

    pub fn get_parent(&self, object: ObjectID) -> Result<ObjectID> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        Ok(obj.parent)
    }

    pub fn set_parent(&mut self, object: ObjectID, parent: ObjectID) -> Result<()> {
        self.objects.get(parent.idx).map_err(obj_error)?; // Make sure parent is valid first
        let obj = self.objects.get_mut(object.idx).map_err(obj_error)?;
        let prev_parent = obj.parent;

        // update child parent -> update previous parent's children -> update new parent's children
        obj.parent = parent;

        let prev_parent = self.objects.get_mut(prev_parent.idx).unwrap(); // This should already be valid so unwrap
        prev_parent.children.remove(&object);

        let new_parent = self.objects.get_mut(parent.idx).unwrap();
        new_parent.children.insert(object);

        Ok(())
    }

    pub fn destroy(&mut self, object: ObjectID) -> Result<()> {
        let obj = self.objects.get(object.idx).map_err(obj_error)?;

        let parent = self.objects.get_mut(obj.parent.idx).unwrap(); // This should already be valid so unwrap
        parent.children.remove(&object);

        self.objects.remove(object.idx).map_err(obj_error)?;

        Ok(())
    }
}

fn obj_error(error: crate::engine::data_structures::error::Error) -> ObjectError {
    match error {
        crate::engine::data_structures::error::Error::ElementRemovedError => ObjectError::DeadObjectError,
        crate::engine::data_structures::error::Error::IndexPointerMismatchError => ObjectError::WorldMismatchError { other: "" },
    }
}

fn comp_error(error: crate::engine::data_structures::error::Error) -> ObjectError {
    match error {
        crate::engine::data_structures::error::Error::ElementRemovedError => ObjectError::DeadComponentError,
        crate::engine::data_structures::error::Error::IndexPointerMismatchError => ObjectError::WorldMismatchError { other: "" },
    }
}