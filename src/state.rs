use glutin::{Event, VirtualKeyCode, WindowEvent};

pub struct State {
    running: bool,
    logical_size: glutin::dpi::LogicalSize,
    should_update_viewport: bool,
}

impl State {
    pub fn new(initial_size: glutin::dpi::LogicalSize) -> Self {
        Self {
            running: true,
            logical_size: initial_size,
            should_update_viewport: false,
        }
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn should_update_viewport(&self) -> bool {
        self.should_update_viewport
    }
    pub fn handle_event(&mut self, event: glutin::Event) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_logical_size) => {
                    self.should_update_viewport = self.logical_size != new_logical_size;
                    self.logical_size = new_logical_size;
                }
                WindowEvent::CloseRequested => self.running = false,
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => self.running = false,
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
