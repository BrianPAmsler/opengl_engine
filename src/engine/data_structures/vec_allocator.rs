use anyhow::{bail, Error};

#[derive(Debug, Clone)]
enum Slot<T> {
    Element { id: usize, value: T },
    Hole { id: usize, next: usize }
}

#[derive(Debug)]
pub struct VecAllocator<T> {
    vec: Vec<Slot<T>>,
    first_hole: usize
}

#[derive(Clone, Copy)]
pub struct AllocationIndex<T> {
    ptr: *const VecAllocator<T>,
    index: usize,
    id: usize
}

impl<T: Clone> VecAllocator<T> {
    #[cfg(test)]
    pub(in crate::engine::data_structures::vec_allocator) fn from_raw(slice: &[Slot<T>], first_hole: usize) -> VecAllocator<T> {
        let mut vec = Vec::new();
        vec.extend_from_slice(slice);

        VecAllocator { first_hole, vec }
    }
}

impl<T> VecAllocator<T> {
    pub fn new() -> VecAllocator<T> {
        let mut vec = Vec::new();
        vec.push(Slot::Hole { id: 0, next: 0 });
        VecAllocator { vec, first_hole: 0 }
    }

    pub fn push(&mut self, value: T) -> AllocationIndex<T> {
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
                let index = AllocationIndex { ptr: self, index: self.first_hole, id };
                self.vec[self.first_hole] = new_slot;

                self.first_hole = next;

                index
            }
        }
    }

    pub fn remove(&mut self, element: AllocationIndex<T>) -> Result<(), Error> {
        let id = match self.vec[element.index] {
            Slot::Hole { .. } => bail!("Element has been removed!"),
            Slot::Element { id, value: _ } => id
        };

        if id != element.id {
            bail!("Element has been removed!")
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

        Ok(())
    }

    pub fn get(&self, element: AllocationIndex<T>) -> Option<&T> {
        match &self.vec[element.index] {
            Slot::Element { id, value } => {
                if *id != element.id {
                    None
                } else {
                    Some(value)
                }
            },
            Slot::Hole { .. } => None,
        }
    }

    pub fn get_mut(&mut self, element: AllocationIndex<T>) -> Option<&mut T> {
        match &mut self.vec[element.index] {
            Slot::Element { id, value } => {
                if *id != element.id {
                    None
                } else {
                    Some(value)
                }
            },
            Slot::Hole { .. } => None,
        }
    }


}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::{Slot, VecAllocator};

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

        let expected = VecAllocator::from_raw(&[
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
            ], 10);

        (0..10).for_each(|n| {
            let entry = allocator.push(n);
            entries.push(entry);
        });

        compare_vecs(&expected, &allocator)?;


        Ok(())
    }
}