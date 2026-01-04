use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum LightColor {
    Red,
    Yellow,
    Green,
}

// Struct representing a real-world traffic light
// It owns its current state and how long that state lasts
struct TrafficLight {
    state: LightColor,
    duration: Duration,
}

struct Controller {
    lights: Vec<TrafficLight>,
    // which light is currently green
    active_index: usize  
}

impl Controller {
    fn tick(&mut self) {
     let active_light = &mut self.lights[self.active_index];
     println!("Active light state: {:?}", active_light.state);

     thread::sleep(active_light.duration);

     active_light.state = next_state(&active_light.state);
     
     active_light.duration = transition_time(&active_light.state);

     if active_light.state == LightColor::Red {
            self.active_index = (self.active_index +1) % self.lights.len();
        }
    }
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

    let mut controller = Controller {
        lights: Vec::new(),
        active_index: 0,
    };

    // Add traffic lights starting in Red
    controller.lights.push(TrafficLight {
        state: LightColor::Red,
        duration: transition_time(&LightColor::Red),
    });
    controller.lights.push(TrafficLight {
        state: LightColor::Red,
        duration: transition_time(&LightColor::Red),
    });
    controller.lights.push(TrafficLight {
        state: LightColor::Red,
        duration: transition_time(&LightColor::Red),
    });

    loop {
        controller.tick();
     }
}
