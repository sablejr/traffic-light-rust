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
        println!("--- TICK ---");

    // Print all lights and their states
    for (i, light) in self.lights.iter().enumerate() {
        let marker = if i == self.active_index { "<-- active" } else { "" };
        println!("Light {}: {:?} {}", i, light.state, marker);
    }

    // copy active_index value so you read then mutate light (this way you are not immutably and mutuably borrowing active_index, this ends the borrow immediately and active_index is independent of self)    
    let active_index = self.active_index;
    let active_light = &mut self.lights[active_index];

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
