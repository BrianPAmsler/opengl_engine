use glfw::Key;

const KEY_COUNT: usize = 348;
const MOUSE_BUTTON_COUNT: usize = 5;

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyState {
    pub is_down: bool,
    pub press: bool,
    pub release: bool
}

pub struct Input {
    keys: Box<[KeyState]>,
    mouse_buttons: Box<[KeyState]>,
    scroll_x: f64,
    scroll_y: f64
}

impl Input {
    pub fn new() -> Input {
        let keys = Box::new([KeyState::default(); KEY_COUNT]);
        let mouse_buttons = Box::new([KeyState::default(); MOUSE_BUTTON_COUNT]);
        Input { keys, mouse_buttons, scroll_x: 0.0, scroll_y: 0.0 }
    }

    pub fn get_mouse_button_state(&self, button: u32) -> KeyState {
        if button as usize > self.mouse_buttons.len() {
            panic!("Invalid mouse button.");
        }

        self.mouse_buttons[button as usize]
    }

    pub fn modify_mouse_button_state(&mut self, button: u32) -> &mut KeyState {
        if button as usize > self.mouse_buttons.len() {
            panic!("Invalid mouse button.");
        }

        &mut self.mouse_buttons[button as usize]
    }

    pub fn modify_all_mouse_button_states<F: FnMut(&mut KeyState)>(&mut self, mut f: F) {
        for state in self.mouse_buttons.iter_mut() {
            f(state)
        }
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

    pub fn get_scroll_x(&self) -> f64 {
        self.scroll_x
    }

    pub fn get_scroll_y(&self) -> f64 {
        self.scroll_y
    }

    pub fn set_scroll_delta(&mut self, x: f64, y: f64) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    pub fn add_scroll_delta(&mut self, x: f64, y: f64) {
        self.scroll_x += x;
        self.scroll_y += y;
    }
}