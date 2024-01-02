use std::cell::RefCell;

use glfw::{fail_on_errors, Glfw, WindowMode, Context, PWindow, GlfwReceiver, WindowEvent};

use anyhow::{Result, anyhow};
use libc::strlen;

use super::GLWrapper;

pub struct Graphics {
    pub gl: GLWrapper,
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>
}

impl Graphics {
    pub fn init(window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<Graphics> {
        let mut glfw = glfw::init(fail_on_errors!())?;
        let (mut window, events) = glfw.create_window(width, height, window_title, window_mode).ok_or(anyhow!("Failed to create GLFW Window!"))?;

        window.make_current();
        window.set_key_polling(true);

        let temp_cell = RefCell::new(window);

        let gl = GLWrapper::init_gl(|t| {
            // freaking c code making me use unsafe...
            unsafe {
                let len = strlen(t as *const i8);
                let s = std::slice::from_raw_parts(t, len);
                temp_cell.borrow_mut().get_proc_address(std::str::from_utf8_unchecked(s))
            }
        })?;

        let window = temp_cell.into_inner();

        Ok(Graphics { gl, glfw, window, events })
    }
}