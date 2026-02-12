use std::{any::Any, marker::PhantomData, ops::{Deref, DerefMut}};

pub struct Resource<T: ?Sized> {
    data: Box<T>
}

impl<T> Resource<T> {
    pub fn new(data: T) -> Resource<T> {
        Resource { data: Box::new(data) }
    }

    pub fn take(self) -> T {
        *self.data
    }
}

impl<T> Deref for Resource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Resource<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub trait ResourceLoader<T: Sized, E: 'static> {
    fn load_resource(&mut self) -> Result<T, E>;
}

struct ResourceData<T: Sized, E: 'static> {
    name: String,
    loader: Box<dyn ResourceLoader<T, E>>,
    data: Option<Resource<T>>
}

pub struct ResourceHandle<T: Sized, E: 'static> {
    ptr: *const ResourceManager,
    idx: usize,
    _pd_t: PhantomData<T>,
    _pd_e: PhantomData<E>
}

impl<T, E> Copy for ResourceHandle<T, E> {}

impl<T, E> Clone for ResourceHandle<T, E> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr, idx: self.idx, _pd_t: PhantomData, _pd_e: PhantomData }
    }
}

pub struct ResourceManager {
    resources: Vec<Box<dyn Any>>
}

struct TestStruct;

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager { resources: Vec::new() }
    }

    pub fn add_resource<T: Any, E: 'static, L: ResourceLoader<T, E> + 'static>(&mut self, loader: L, name: String) -> ResourceHandle<T, E> {
        let handle = ResourceHandle::<T, E> {
            ptr: self as _,
            idx: self.resources.len(),
            _pd_t: PhantomData,
            _pd_e: PhantomData
        };

        let data: ResourceData<T, E> = ResourceData {
            name,
            loader: Box::new(loader),
            data: None
        };

        self.resources.push(Box::new(data));

        handle
    }

    pub fn get_resource<T: 'static, E: 'static>(&mut self, handle: ResourceHandle<T, E>) -> Result<&mut Resource<T>, E> {
        if handle.ptr != self as *const _ {
            panic!("Invalid hanlde!")
        }

        let box_ = &mut self.resources[handle.idx];
        let ref_: &mut ResourceData<T, E> = box_.downcast_mut().unwrap();

        if ref_.data.is_none() {
            ref_.data = Some(Resource::new(ref_.loader.load_resource()?));
        }

        Ok(ref_.data.as_mut().unwrap())
    }

    pub fn is_loaded<T: 'static, E: 'static>(&self, handle: ResourceHandle<T, E>) -> bool {
        let box_ = &self.resources[handle.idx];
        let ref_: &ResourceData<T, E> = box_.downcast_ref().unwrap();

        ref_.data.is_some()
    }

    pub fn unload_resource<T: 'static, E: 'static>(&mut self, handle: ResourceHandle<T, E>) {
        let box_ = &mut self.resources[handle.idx];
        let ref_: &mut ResourceData<T, E> = box_.downcast_mut().unwrap();

        ref_.data.take();
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, fs::File, io::{Read, Write}, path::PathBuf, rc::Rc};

    use tempfile::{NamedTempFile};

    use super::{ResourceLoader, ResourceManager};

    struct TestSructLoader {
        count: u32
    }
    impl ResourceLoader<u32, ()> for TestSructLoader {
        fn load_resource(&mut self) -> Result<u32, ()> {
            self.count += 1;
            Ok(self.count)
        }
    }

    struct TestStruct {}

    #[test]
    fn resource_dyn_test() {
        let mut manager = ResourceManager::new();

        let handle = manager.add_resource(TestSructLoader { count: 4 }, "Test".to_owned());

        let resource = manager.get_resource(handle);
        let data1 = **resource.unwrap();

        let resource = manager.get_resource(handle);
        let data2 = **resource.unwrap();

        manager.unload_resource(handle);

        let resource = manager.get_resource(handle);
        let data3 = **resource.unwrap();

        assert_eq!(data1, 5);
        assert_eq!(data2, 5);
        assert_eq!(data3, 6);
    }

    struct FileLoader {
        path: PathBuf,
        read_count: Rc<RefCell<u32>>
    }

    impl ResourceLoader<String, std::io::Error> for FileLoader {
        fn load_resource(&mut self) -> Result<String, std::io::Error> {
            *self.read_count.borrow_mut() += 1;
            let mut file = File::open(&self.path)?;

            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            Ok(buf)
        }
    }

    #[test]
    fn resource_file_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;

        const FILE_CONTENTS: &'static str = "
            This is a test file.
            here is some text.
        ";

        write!(file.as_file_mut(), "{}", FILE_CONTENTS)?;

        let path = file.path().to_owned();

        let mut manager = ResourceManager::new();
        let read_count = Rc::new(RefCell::new(0));
        let handle = manager.add_resource(FileLoader { path, read_count: read_count.clone() }, "Test File".to_owned());

        let contents = &**manager.get_resource(handle)?;
        assert_eq!(FILE_CONTENTS, contents);
        assert_eq!(*read_count.borrow(), 1);

        let contents = &**manager.get_resource(handle)?;
        assert_eq!(FILE_CONTENTS, contents);
        assert_eq!(*read_count.borrow(), 1);

        manager.unload_resource(handle);
        let contents = &**manager.get_resource(handle)?;
        assert_eq!(FILE_CONTENTS, contents);
        assert_eq!(*read_count.borrow(), 2);


        Ok(())
    }
}