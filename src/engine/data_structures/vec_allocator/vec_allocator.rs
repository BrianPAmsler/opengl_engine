use std::ops::Deref;

use super::error::Result;

#[derive(Debug, Clone)]
enum Slot<T> {
    Element { id: usize, value: T },
    Hole { id: usize, next: usize }
}

#[derive(Debug)]
pub struct VecAllocator<T> {
    vec: Box<Vec<Slot<T>>>, // Put vec in a box so I have a pointer that I can compare with that won't break if the VecAllocator gets moved
    first_hole: usize,
    count: usize
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AllocationIndex {
    ptr: *const (),
    index: usize,
    id: usize
}

impl AllocationIndex {
    pub fn null() -> AllocationIndex {
        AllocationIndex { ptr: std::ptr::null(), index: 0, id: 0 }
    }

    pub fn as_raw(self) -> (*const(), usize, usize) {
        (self.ptr, self.index, self.id)
    }

    pub fn ptr_eq<T>(&self, allocator: &VecAllocator<T>) -> bool {
        self.ptr == allocator.vec.deref() as *const Vec<Slot<T>> as *const ()
    }
}

impl<T: Clone> VecAllocator<T> {
    #[cfg(test)]
    fn from_raw(slice: &[Slot<T>]) -> VecAllocator<T> {
        let mut vec = Box::new(Vec::new());
        vec.extend_from_slice(slice);

        let mut first_hole = vec.len();
        let mut count = 0;
        for (i, slot) in vec.iter().enumerate() {
            match slot {
                Slot::Hole { .. } => {
                    if first_hole >= vec.len() {
                        first_hole = i;
                    }
                },
                Slot::Element { .. } => count += 1
            }
        }

        if first_hole == vec.len() {
            panic!("No hole!");
        }

        VecAllocator { first_hole, vec, count }
    }
}

impl<T> VecAllocator<T> {
    pub fn new() -> VecAllocator<T> {
        let mut vec = Box::new(Vec::new());
        vec.push(Slot::Hole { id: 0, next: 0 });
        VecAllocator { vec, first_hole: 0, count: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.vec.len()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn insert(&mut self, value: T) -> AllocationIndex {
        match self.vec[self.first_hole] {
            Slot::Element { .. } => panic!("first_hole is invalid! This should be impossible, there is a bug somewhere!"),
            Slot::Hole { id, next } => {
                let new_slot = Slot::Element { id, value };

                // If there is only one hole, add a new hole at the end
                let mut next = next;
                if next <= self.first_hole {
                    next = self.vec.len();
                    let new_hole = Slot::Hole { id: 0, next };
                    self.vec.push(new_hole);
                }

                // Place the new element
                let index = AllocationIndex { ptr: self.vec.deref() as *const Vec<Slot<T>> as *const (), index: self.first_hole, id };
                self.vec[self.first_hole] = new_slot;

                self.first_hole = next;
                self.count += 1;

                index
            }
        }
    }

    pub fn remove(&mut self, element: AllocationIndex) -> Result<()> {
        if !element.ptr_eq(self) {
            return Err(super::error::Error::IndexPointerMismatchError);
        }

        let id = match self.vec[element.index] {
            Slot::Hole { .. } => return Err(super::error::Error::ElementRemovedError),
            Slot::Element { id, value: _ } => id
        };

        if id != element.id {
            return Err(super::error::Error::ElementRemovedError);
        }

        let mut next = self.first_hole;
        let mut prev = None;

        while next < element.index {
            match &self.vec[next] {
                Slot::Element { .. } => panic!("something is fucked with VecAllocator!"),
                Slot::Hole { id: _, next: next_hole } => {
                    prev = Some(next);

                    if *next_hole == next {
                        break;
                    }

                    next = *next_hole;
                }
            }
        }

        match prev {
            Some(prev) => {
                match &mut self.vec[prev] {
                    Slot::Element { .. } => panic!("something is fucked with VecAllocator!"),
                    Slot::Hole { id: _, next } => {
                        *next = element.index;
                    }
                }
            },
            None => ()
        }

        let new_hole = Slot::Hole { id: id + 1, next };
        self.vec[element.index] = new_hole;

        if element.index < self.first_hole {
            self.first_hole = element.index;
        }

        self.count -= 1;

        Ok(())
    }

    pub fn get(&self, element: AllocationIndex) -> Result<&T> {
        if !element.ptr_eq(self) {
            return Err(super::error::Error::IndexPointerMismatchError);
        }

        match &self.vec[element.index] {
            Slot::Element { id, value } => {
                if *id != element.id {
                    Err(super::error::Error::ElementRemovedError)
                } else {
                    Ok(value)
                }
            },
            Slot::Hole { .. } => Err(super::error::Error::ElementRemovedError),
        }
    }

    pub fn get_mut(&mut self, element: AllocationIndex) -> Result<&mut T> {
        if !element.ptr_eq(self) {
            return Err(super::error::Error::IndexPointerMismatchError);
        }
        
        match &mut self.vec[element.index] {
            Slot::Element { id, value } => {
                if *id != element.id {
                    Err(super::error::Error::ElementRemovedError)
                } else {
                    Ok(value)
                }
            },
            Slot::Hole { .. } => Err(super::error::Error::ElementRemovedError),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { allocator: self, index: 0 }
    }
}

pub struct Iter<'a, T> {
    allocator: &'a VecAllocator<T>,
    index: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (AllocationIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.allocator.vec.len() {
            match &self.allocator.vec[self.index] {
                Slot::Element { id, value, .. } => {
                    let index = AllocationIndex { ptr: self.allocator as *const _ as *const (), index: self.index, id: *id };
                    self.index += 1;
                    return Some((index, &value));
                },
                Slot::Hole { .. } => self.index += 1,
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::{Slot, VecAllocator};

    fn insert<T: std::fmt::Debug>(vec: &mut Vec<Slot<T>>, value: T) {
        let mut slot = None;
        let mut i = 0;
        while slot.is_none() && i < vec.len() {
            let next_slot = &vec[i];

            match next_slot {
                Slot::Hole { id, next } => slot = Some((i, *id, *next)),
                _ => ()
            }

            i += 1;
        }

        match slot {
            Some((i, id, next)) => {
                vec[i] = Slot::Element { id, value };

                // Add hole at end if this was the last hole
                if next <= i {
                    vec.push(Slot::Hole { id: 0, next: vec.len() });
                }
            },
            None => panic!("Malformed vec!")
        }
    }

    fn remove<T: std::fmt::Debug>(vec: &mut Vec<Slot<T>>, index: usize) {
        let id = match &vec[index] {
            Slot::Element { id, .. } => *id + 1,
            _ => panic!("already hole!")
        };

        let new_hole = Slot::Hole { id, next: 0 };

        vec[index] = new_hole;

        // Update all holes
        let mut next_hole = vec.len() - 1;
        for (i, slot) in vec.iter_mut().enumerate().rev() {
            match slot {
                Slot::Element { .. } => (),
                Slot::Hole { next , .. } => {
                    *next = next_hole;
                    next_hole = i;
                },
            }
        }
    }

    fn compare_vecs<T: std::fmt::Debug>(a: &VecAllocator<T>, b: &VecAllocator<T>) -> Result<(), String> {
        let string_a = format!("{:?}", *a);
        let string_b = format!("{:?}", *b);

        if string_a != string_b {
            eprintln!("A: {}", string_a);
            eprintln!("B: {}", string_b);
            return Err("Not equal!".into());
        }

        Ok(())
    }

    #[test]
    pub fn vec_allocator() -> Result<(), String> {
        let mut entries = Vec::new();
        let mut allocator = VecAllocator::new();
        let mut test_vec = Vec::new();

        let ten_elements = [
            Slot::Element { id: 0, value: 0 },
            Slot::Element { id: 0, value: 1 },
            Slot::Element { id: 0, value: 2 },
            Slot::Element { id: 0, value: 3 },
            Slot::Element { id: 0, value: 4 },
            Slot::Element { id: 0, value: 5 },
            Slot::Element { id: 0, value: 6 },
            Slot::Element { id: 0, value: 7 },
            Slot::Element { id: 0, value: 8 },
            Slot::Element { id: 0, value: 9 },
            Slot::Hole { id: 0, next: 10 }
            ];

        let expected = VecAllocator::from_raw(&ten_elements);
        test_vec.extend(ten_elements.into_iter());

        (0..10).for_each(|n| {
            let entry = allocator.insert(n);
            entries.push(entry);
        });

        compare_vecs(&expected, &allocator)?;

        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let choice: f32 = rng.gen();

            if entries.len() == 0 || choice > 0.45 {
                let value = rng.gen_range(0..100);

                println!("Inserting value {}...", value);
                insert(&mut test_vec, value);
                entries.push(allocator.insert(value));

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            } else {
                let index = rng.gen_range(0..entries.len());
                let entry = entries.remove(index);

                println!("Removing at index {}...", entry.index);
                remove(&mut test_vec, entry.index);
                allocator.remove(entry).unwrap();

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            }
        }
        
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let choice: f32 = rng.gen();

            if entries.len() == 0 || choice > 0.9 {
                let value = rng.gen_range(0..100);

                println!("Inserting value {}...", value);
                insert(&mut test_vec, value);
                entries.push(allocator.insert(value));

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            } else {
                let index = rng.gen_range(0..entries.len());
                let entry = entries.remove(index);

                println!("Removing at index {}...", entry.index);
                remove(&mut test_vec, entry.index);
                allocator.remove(entry).unwrap();

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            }
        }

        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let choice: f32 = rng.gen();

            if entries.len() == 0 || choice > 0.1 {
                let value = rng.gen_range(0..100);

                println!("Inserting value: {}...", value);
                insert(&mut test_vec, value);
                entries.push(allocator.insert(value));

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            } else {
                let index = rng.gen_range(0..entries.len());
                let entry = entries.remove(index);

                println!("Removing at index: {}...", entry.index);
                remove(&mut test_vec, entry.index);
                allocator.remove(entry).unwrap();

                let expected = VecAllocator::from_raw(&test_vec);
                compare_vecs(&expected, &allocator)?;
            }
        }

        Ok(())
    }
}