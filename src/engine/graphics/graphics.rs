use std::{cell::RefCell, ops::Deref};

use glfw::{fail_on_errors, Glfw, Context, PWindow, GlfwReceiver, WindowEvent, WindowMode};

use anyhow::{Result, anyhow};
use libc::strlen;

use super::GLWrapper;

pub struct Graphics {
    gl: GLWrapper,
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>
}

impl Graphics {
    pub fn init(window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<Graphics> {
        let mut glfw = glfw::init(fail_on_errors!())?;
        let (mut window, events) = glfw.create_window(width, height, window_title, window_mode).ok_or(anyhow!("Failed to create GLFW Window!"))?;

        window.make_current();
        window.set_key_polling(true);

        let temp_cell = RefCell::new(window);

        let gl = GLWrapper::init_gl(|t| {
            // freaking c strings...
            unsafe {
                let len = strlen(t as *const i8);
                let s = std::slice::from_raw_parts(t, len);
                temp_cell.borrow_mut().get_proc_address(std::str::from_utf8_unchecked(s))
            }
        })?;

        let window = temp_cell.into_inner();

        Ok(Graphics { gl, glfw, window, events })
    }

    pub fn process_frame(&mut self) {
        // Swap front and back buffers
        self.window.swap_buffers();

        // Poll for and process events
        self.glfw.poll_events();
    }

    pub fn flush_messages(&self) -> std::vec::IntoIter<(f64, WindowEvent)> {
        glfw::flush_messages(&self.events).collect::<Vec<(f64, WindowEvent)>>().into_iter()
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value);
    }

    // This will be deleted once glfw is properly wrapped
    pub fn __get_glfw(&self) -> &Glfw {
        &self.glfw
    }

    // This will be deleted once glfw is properly wrapped
    pub fn __get_glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    // This will be deleted once window is properly wrapped
    pub fn __get_window(&self) -> &PWindow {
        &self.window
    }

    // This will be deleted once window is properly wrapped
    pub fn __get_window_mut(&mut self) -> &mut PWindow {
        &mut self.window
    }
}

impl Deref for Graphics {
    type Target = GLWrapper;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}