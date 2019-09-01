use glutin::{Event, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent};

#[derive(Copy, Debug, Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub struct State {
    running: bool,
    logical_size: glutin::dpi::LogicalSize,
    should_update_viewport: bool,
    position: Position,
}

impl State {
    pub fn new(initial_size: glutin::dpi::LogicalSize) -> Self {
        Self {
            running: true,
            logical_size: initial_size,
            should_update_viewport: false,
            position: Position { row: 0, column: 0 },
        }
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn should_update_viewport(&self) -> bool {
        self.should_update_viewport
    }
    pub fn get_position(&self) -> Position {
        self.position
    }
    fn go_down(&mut self, y: f32) {
        self.position.row = self
            .position
            .row
            .checked_add(3 * (-y) as usize)
            .unwrap_or(0);
    }
    fn go_up(&mut self, y: f32) {
        self.position.row = self.position.row.checked_sub(3 * y as usize).unwrap_or(0);
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
                WindowEvent::MouseWheel { delta, phase, .. } => match (delta, phase) {
                    // TODO: support trackpad
                    (MouseScrollDelta::LineDelta(_x, y), TouchPhase::Moved) => {
                        if y > 0. {
                            self.go_up(y);
                        } else {
                            self.go_down(y);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
