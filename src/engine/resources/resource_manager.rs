use std::{any::Any, marker::PhantomData};

pub struct Resource<T: ?Sized> {
    data: Box<T>
}

impl<T> Resource<T> {
    pub fn new(data: T) -> Resource<T> {
        Resource { data: Box::new(data) }
    }

    pub fn borrow(&self) -> &T {
        &self.data
    }

    pub fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn take(self) -> T {
        *self.data
    }
}

pub trait ResourceLoader<T: ?Sized> {
    fn load_resource(&mut self) -> T;
}

struct ResourceData<T: ?Sized> {
    name: String,
    loader: Box<dyn ResourceLoader<T>>,
    data: Option<Resource<T>>
}

#[derive(Clone, Copy)]
pub struct ResourceHandle<T: ?Sized> {
    ptr: *const ResourceManager,
    idx: usize,
    _pd: PhantomData<T>
}

pub struct ResourceManager {
    resources: Vec<Box<dyn Any>>
}

struct TestStruct;

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager { resources: Vec::new() }
    }

    pub fn add_resource<T: Any, L: ResourceLoader<T> + 'static>(&mut self, loader: L, name: String) -> ResourceHandle<T> {
        let handle = ResourceHandle {
            ptr: self as _,
            idx: self.resources.len(),
            _pd: PhantomData,
        };

        let data: ResourceData<T> = ResourceData {
            name,
            loader: Box::new(loader),
            data: None
        };

        self.resources.push(Box::new(data));

        handle
    }

    pub fn get_resource<T: 'static>(&mut self, handle: ResourceHandle<T>) -> &mut Resource<T> {
        if handle.ptr != self as *const _ {
            panic!("Invalid hanlde!")
        }

        let box_ = &mut self.resources[handle.idx];
        let ref_: &mut ResourceData<T> = box_.downcast_mut().unwrap();

        if ref_.data.is_none() {
            ref_.data = Some(Resource::new(ref_.loader.load_resource()));
        }

        ref_.data.as_mut().unwrap()
    }

    pub fn is_loaded<T: 'static>(&self, handle: ResourceHandle<T>) -> bool {
        let box_ = &self.resources[handle.idx];
        let ref_: &ResourceData<T> = box_.downcast_ref().unwrap();

        ref_.data.is_some()
    }

    pub fn unload_resource<T: 'static>(&mut self, handle: ResourceHandle<T>) {
        let box_ = &mut self.resources[handle.idx];
        let ref_: &mut ResourceData<T> = box_.downcast_mut().unwrap();

        ref_.data.take();
    }
}

#[cfg(test)]
mod tests {
    use super::{ResourceLoader, ResourceManager};

    struct TestSructLoader {
        count: u32
    }
    impl ResourceLoader<u32> for TestSructLoader {
        fn load_resource(&mut self) -> u32 {
            self.count += 1;
            self.count
        }
    }

    struct TestStruct {}

    #[test]
    fn resource_dyn_test() {
        let mut manager = ResourceManager::new();

        let handle = manager.add_resource(TestSructLoader { count: 4 }, "Test".to_owned());

        let resource = manager.get_resource(handle);
        let data1 = *resource.borrow();

        let resource = manager.get_resource(handle);
        let data2 = *resource.borrow();

        manager.unload_resource(handle);

        let resource = manager.get_resource(handle);
        let data3 = *resource.borrow();

        assert_eq!(data1, 5);
        assert_eq!(data2, 5);
        assert_eq!(data3, 6);
    }
}