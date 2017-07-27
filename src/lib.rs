extern crate glium;

use glium::Display;
pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::MouseButton as MouseButton;
use glium::glutin::MouseScrollDelta;
use glium::glutin::KeyboardInput;
use glium::glutin::EventsLoop;
use glium::glutin::WindowEvent::{ MouseMoved, MouseInput, MouseWheel, ReceivedCharacter };
use glium::glutin::WindowEvent::KeyboardInput as WKeyboardInput;
use glium::glutin::Event::WindowEvent;
use glium::glutin::ElementState::{ Pressed, Released };
use glium::glutin::CursorState::{ Normal, Hide };


/// struct for abstracting the state for all the inputs
pub struct Input {
    /// The position of the mouse
    pub mouse_pos: (i32, i32),
    /// The difference in mouse position from the last frame
    pub mouse_delta: (f32, f32),
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
    event_loop: EventsLoop,
}

impl Input {
    /// Creates a new Input instance
    pub fn new() -> Input {
        Input {
            mouse_pos : (0, 0),
            mouse_delta : (0f32, 0f32),
            mouse_wheel_delta: (0f32, 0f32),
            keys_down : Vec::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            characters_down: Vec::new(),
            mouse_btns_down: Vec::new(),
            mouse_btns_pressed: Vec::new(),
            mouse_btns_released: Vec::new(),
            hide_mouse: true,
            cursor_grabbed: false,
            event_loop: EventsLoop::new(),
        }
    }

    /// This method updates the state of the inputs
    pub fn update_inputs(&mut self, display: &Display) {
        let window = display.gl_window();
        let (width, height) = window.get_inner_size().unwrap_or((800, 600));
        let hidpi_factor = window.hidpi_factor();
        
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

        let event_loop = &mut self.event_loop;
        let keys_down = &mut self.keys_down;
        let keys_pressed = &mut self.keys_pressed;
        let keys_released = &mut self.keys_released;
        let mouse_delta = &mut self.mouse_delta;
        let mouse_pos = &mut self.mouse_pos;
        let mouse_btns_down = &mut self.mouse_btns_down;
        let mouse_btns_pressed = &mut self.mouse_btns_pressed;
        let mouse_btns_released = &mut self.mouse_btns_released;
        let mouse_wheel_delta = &mut self.mouse_wheel_delta;
        let characters_down = &mut self.characters_down;

        // polling and handling the events received by the display
        event_loop.poll_events(|event| {
            if let WindowEvent { event, .. } = event {
                match event {
                    WKeyboardInput {
                        input: KeyboardInput { state: Pressed, virtual_keycode: vkey, .. },
                        ..
                    } => {
                        keys_down.push(vkey.unwrap());
                        keys_pressed.push(vkey.unwrap());
                    },
                    WKeyboardInput {
                        input: KeyboardInput { state: Released, virtual_keycode: vkey, .. },
                        ..
                    } => {
                        keys_down.retain(|&k| k != vkey.unwrap());
                        keys_released.push(vkey.unwrap());
                    },
                    MouseMoved { position: (x, y), .. } => {
                        let mouse_diff = ((width / 2) as i32 - (x as f32 / hidpi_factor) as i32,
                                          (height / 2) as i32 - (y as f32 / hidpi_factor) as i32);
                        mouse_delta.0 = (mouse_diff.0 as f32)/(width as f32);
                        mouse_delta.1 = (mouse_diff.1 as f32)/(height as f32);
                        (*mouse_pos) = (x as i32, y as i32);
                    },
                    MouseInput { state: Pressed, button: btn, .. } => {
                        mouse_btns_down.push(btn);
                        mouse_btns_pressed.push(btn);
                    },
                    MouseInput { state: Released, button: btn, .. } => {
                        mouse_btns_down.retain(|&mb| mb != btn);
                        mouse_btns_released.push(btn);
                    },
                    MouseWheel { delta, .. } => {
                        (*mouse_wheel_delta) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(x, y) => (x, y),
                        };
                    },
                    ReceivedCharacter(c) => characters_down.push(c),
                    _ => ()
                }
            }
        });

        if self.hide_mouse {
            // set the mouse to the centre of the screen
            if self.cursor_grabbed {
                window.set_cursor_state(Hide).ok();
                self.cursor_grabbed = false;
            }
            let _ = window.set_cursor_position((width / 2) as i32, (height / 2) as i32);
        } else {
            if !self.cursor_grabbed {
                window.set_cursor_state(Normal).ok();
                self.cursor_grabbed = true;
            }
        }
    }
}