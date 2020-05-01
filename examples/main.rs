use volition::Input;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use std::thread;
use std::time::Duration;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let mut input = Input::new();

    let mut counter = 0;

    event_loop.run(move |event, _, control_flow| {
        thread::sleep(Duration::new(1u64, 0u32));
        input.update_inputs(&window, event);
        println!("{:?}", input.keys_down);
        println!("{:?}", input.mouse_axis_motion);
        counter += 1;
        if counter > 4 {
            *control_flow = ControlFlow::Exit;
        }
    });
}
