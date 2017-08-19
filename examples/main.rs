extern crate volition;
extern crate winit;

use volition::Input;
use winit::{EventsLoop, Window};

fn main() {
    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    let mut input = Input::from_existing(events_loop);

    for _ in 0..10 {
        input.update_inputs(&window);
        println!("{:?}", input.keys_down);
    }
}
