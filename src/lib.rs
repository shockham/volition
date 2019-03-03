/*!
    Minimalist input lib based on [winit](https://github.com/tomaka/winit).
*/

#![deny(missing_docs)]

extern crate winit;

use winit::dpi::LogicalPosition;
use winit::DeviceEvent::MouseMotion;
use winit::ElementState::{Pressed, Released};
use winit::Event::{DeviceEvent, WindowEvent};
use winit::EventsLoop;
use winit::KeyboardInput;
pub use winit::MouseButton;
use winit::MouseScrollDelta;
pub use winit::VirtualKeyCode as Key;
use winit::Window;
use winit::WindowEvent::KeyboardInput as WKeyboardInput;
use winit::WindowEvent::{AxisMotion, CursorMoved, MouseInput, MouseWheel, ReceivedCharacter};

/// struct for abstracting the state for all the inputs
pub struct Input {
    /// The position of the mouse
    pub mouse_pos: (f32, f32),
    /// The difference in mouse position from the last frame
    pub mouse_delta: (f32, f32),
    /// The raw difference in mouse position from the last frame
    pub raw_mouse_delta: (f32, f32),
    /// The mouse axis motion for the last frame
    pub mouse_axis_motion: (f32, f32),
    /// The difference in position of the mouse when from the previous frame
    pub mouse_wheel_delta: (f32, f32),
    /// The keys that are currently pressed down
    pub keys_down: Vec<Key>,
    /// The keys that have been pressed on this frame
    pub keys_pressed: Vec<Key>,
    /// The keys that have been released on this frame
    pub keys_released: Vec<Key>,
    /// Characters received that are pressed down
    pub characters_down: Vec<char>,
    /// The mouse buttons that are currently pressed down
    pub mouse_btns_down: Vec<MouseButton>,
    /// The mouse buttons that have been pressed down on this frame
    pub mouse_btns_pressed: Vec<MouseButton>,
    /// The mouse buttons that have been release on this frame
    pub mouse_btns_released: Vec<MouseButton>,
    /// Whether to show or hide the mouse
    pub hide_mouse: bool,
    /// Internal field to track if the cursor is grabbed
    cursor_grabbed: bool,
    /// Event loop for the window
    events_loop: EventsLoop,
}

impl Default for Input {
    /// Create a default instance of Input
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    /// Creates a new Input instance
    pub fn new() -> Input {
        let events_loop = EventsLoop::new();

        Input {
            mouse_pos: (0f32, 0f32),
            mouse_delta: (0f32, 0f32),
            raw_mouse_delta: (0f32, 0f32),
            mouse_axis_motion: (0f32, 0f32),
            mouse_wheel_delta: (0f32, 0f32),
            keys_down: Vec::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            characters_down: Vec::new(),
            mouse_btns_down: Vec::new(),
            mouse_btns_pressed: Vec::new(),
            mouse_btns_released: Vec::new(),
            hide_mouse: true,
            cursor_grabbed: false,
            events_loop,
        }
    }

    /// Creates a new Input instance from existing events_loop
    pub fn from_existing(events_loop: EventsLoop) -> Input {
        Input {
            mouse_pos: (0f32, 0f32),
            mouse_delta: (0f32, 0f32),
            raw_mouse_delta: (0f32, 0f32),
            mouse_axis_motion: (0f32, 0f32),
            mouse_wheel_delta: (0f32, 0f32),
            keys_down: Vec::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            characters_down: Vec::new(),
            mouse_btns_down: Vec::new(),
            mouse_btns_pressed: Vec::new(),
            mouse_btns_released: Vec::new(),
            hide_mouse: true,
            cursor_grabbed: false,
            events_loop,
        }
    }

    /// This method updates the state of the inputs
    pub fn update_inputs(&mut self, window: &Window) {
        let (width, height): (u32, u32) = window.get_inner_size().unwrap().into();
        let h_width = (width as u32 / 2u32) as f32;
        let h_height = (height / 2u32) as f32;

        let hidpi_factor = window.get_hidpi_factor();

        // reset properties
        {
            // reset the delta in case the mouse does not move
            self.mouse_delta = (0f32, 0f32);
            self.mouse_wheel_delta = (0f32, 0f32);

            // keys pressed is only for a single frame so clear
            self.keys_pressed.clear();
            self.keys_released.clear();
            self.mouse_btns_pressed.clear();
            self.mouse_btns_released.clear();
            self.characters_down.clear();
        }

        let keys_down = &mut self.keys_down;
        let keys_pressed = &mut self.keys_pressed;
        let keys_released = &mut self.keys_released;
        let mouse_delta = &mut self.mouse_delta;
        let raw_mouse_delta = &mut self.raw_mouse_delta;
        let mouse_axis_motion = &mut self.mouse_axis_motion;
        let mouse_pos = &mut self.mouse_pos;
        let mouse_btns_down = &mut self.mouse_btns_down;
        let mouse_btns_pressed = &mut self.mouse_btns_pressed;
        let mouse_btns_released = &mut self.mouse_btns_released;
        let mouse_wheel_delta = &mut self.mouse_wheel_delta;
        let characters_down = &mut self.characters_down;

        // polling and handling the events received by the display
        self.events_loop.poll_events(|event| match event {
            WindowEvent { event, .. } => match event {
                WKeyboardInput {
                    input:
                        KeyboardInput {
                            state: Pressed,
                            virtual_keycode: vkey,
                            ..
                        },
                    ..
                } => {
                    if let Some(key) = vkey {
                        keys_down.push(key);
                        keys_pressed.push(key);
                    }
                }
                WKeyboardInput {
                    input:
                        KeyboardInput {
                            state: Released,
                            virtual_keycode: vkey,
                            ..
                        },
                    ..
                } => {
                    if let Some(key) = vkey {
                        keys_down.retain(|&k| k != key);
                        keys_released.push(key);
                    }
                }
                CursorMoved { position: pos, .. } => {
                    let (x, y): (i32, i32) = pos.into();
                    mouse_delta.0 = (h_width - x as f32) / width as f32;
                    mouse_delta.1 = (h_height - y as f32) / height as f32;
                    (*mouse_pos) = (x as f32, y as f32);
                }
                AxisMotion { axis, value, .. } => match axis {
                    0 => {
                        mouse_axis_motion.0 =
                            (h_width - (value / hidpi_factor) as f32) / width as f32
                    }
                    1 => {
                        mouse_axis_motion.1 =
                            (h_height - (value / hidpi_factor) as f32) / height as f32
                    }
                    _ => {}
                },
                MouseInput {
                    state: Pressed,
                    button: btn,
                    ..
                } => {
                    mouse_btns_down.push(btn);
                    mouse_btns_pressed.push(btn);
                }
                MouseInput {
                    state: Released,
                    button: btn,
                    ..
                } => {
                    mouse_btns_down.retain(|&mb| mb != btn);
                    mouse_btns_released.push(btn);
                }
                MouseWheel { delta, .. } => {
                    (*mouse_wheel_delta) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x, y),
                        MouseScrollDelta::PixelDelta(pos) => {
                            let (x, y): (i32, i32) = pos.into();
                            (x as f32, y as f32)
                        }
                    };
                }
                ReceivedCharacter(c) => characters_down.push(c),
                _ => (),
            },
            DeviceEvent { event, .. } => {
                if let MouseMotion { delta } = event {
                    (*raw_mouse_delta) = (delta.0 as f32, delta.1 as f32);
                }
            }
            _ => (),
        });

        if self.hide_mouse {
            // set the mouse to the centre of the screen
            if self.cursor_grabbed {
                window.hide_cursor(true);
                self.cursor_grabbed = false;
            }
            let logi_pos = LogicalPosition::new(h_width as f64, h_height as f64);
            let _ = window.set_cursor_position(logi_pos);
        } else if !self.cursor_grabbed {
            window.hide_cursor(false);
            self.cursor_grabbed = true;
        }
    }
}
