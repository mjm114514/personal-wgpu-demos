use winit::event::{
    ElementState, KeyboardInput, WindowEvent, VirtualKeyCode,
    MouseButton,
};

pub struct Controller {
    pub speed: f32,

    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,

    pub dragged: bool,

    pub current_cursor: (f64, f64),
    pub last_cursor: (f64, f64),
}

pub trait ControllerUpdate {
    fn update(&mut self, controller: &Controller, duration: f32);
}

impl Controller {
    pub fn update_all(&mut self, update_objects: &mut [&mut dyn ControllerUpdate], duration: f32) {
        for object in update_objects {
            object.update(self, duration);
        }

        self.last_cursor = self.current_cursor;
    }

    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            current_cursor: (0.0, 0.0),
            last_cursor: (0.0, 0.0),
            dragged: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let press_state = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.up_pressed = press_state;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.down_pressed = press_state;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.left_pressed = press_state;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.right_pressed = press_state;
                        true
                    }
                    _ => false
                }
            }

            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                let press_state = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.dragged = press_state;
                        true
                    }
                    _ => false
                }
            }

            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                self.current_cursor = (*position).into();
                true
            }

            _ => false
        }
    }
}