use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum LightColor {
    Red,
    Yellow,
    Green,
}

//The function only needs to inspect the state to determine the next one
fn next_state(current: &LightColor) -> LightColor {
    match current {
        LightColor::Red => LightColor::Green,
        LightColor::Green => LightColor::Yellow,
        LightColor::Yellow => LightColor::Red,
    }
}

fn transition_time(current: &LightColor) -> Duration {
    match current {
        LightColor::Red => Duration::from_secs(3),
        LightColor::Green => Duration::from_secs(3),
        LightColor::Yellow => Duration::from_secs(1),
    }
}




fn main() {
    let mut state = LightColor::Red;
    loop {
        println!("Current state: {:?}", state);
        thread::sleep(transition_time(&state));
        state = next_state(&state);
    }
}
