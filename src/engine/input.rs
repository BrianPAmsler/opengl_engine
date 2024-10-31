use glfw::Key;

const KEY_COUNT: usize = 348;

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyState {
    pub is_down: bool,
    pub press: bool,
    pub release: bool
}

pub struct Input {
    keys: Box<[KeyState]>
}

impl Input {
    pub fn new() -> Input {
        let keys = Box::new([KeyState::default(); KEY_COUNT]);
        Input { keys }
    }

    pub fn get_key_state(&self, key: Key) -> KeyState {
        match  key {
            Key::Unknown => panic!("Unknown key"),
            _ => self.keys[key as usize]
        }
    }

    pub fn modify_key_state(&mut self, key: Key) -> &mut KeyState {
        match  key {
            Key::Unknown => panic!("Unknown key"),
            _ => &mut self.keys[key as usize]
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, KeyState> {
        self.keys.iter()
    }

    pub fn modify_all_key_states<F: FnMut(&mut KeyState)>(&mut self, mut f: F) {
        for state in self.keys.iter_mut() {
            f(state)
        }
    }
}