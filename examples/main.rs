extern crate volition;

use volition::Input;

fn main() {
    let mut input = Input::new(); 

    for _ in 0..100 {
        input.update_inputs();
        println!("{:?}", input.keys_down);
    }
}
