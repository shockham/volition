extern crate volition;
extern crate winit;

use volition::Input;
use winit::{EventsLoop, Window};

use std::thread;
use std::time::Duration;

fn main() {
    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    let mut input = Input::from_existing(events_loop);

    for _ in 0..10 {
        thread::sleep(Duration::new(1u64, 0u32));
        input.update_inputs(&window);
        println!("{:?}", input.keys_down);
    }
}
