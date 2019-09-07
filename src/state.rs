use glutin::event::KeyboardInput;

use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event::{MouseScrollDelta, TouchPhase};

use glutin::event_loop::ControlFlow;

#[derive(Copy, Debug, Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub struct State {
    position: Position,
}
impl State {
    pub fn new() -> Self {
        Self {
            position: Position { row: 0, column: 0 },
        }
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
    pub fn handle_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_keycode, state) {
                    (VirtualKeyCode::Escape, _) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_x, y),
                    phase: TouchPhase::Moved,
                    ..
                } => {
                    if y > &0. {
                        self.go_up(*y);
                    } else {
                        self.go_down(*y);
                    }
                }
                _ => *control_flow = ControlFlow::Wait,
            },
            _ => (),
        }
    }
}
