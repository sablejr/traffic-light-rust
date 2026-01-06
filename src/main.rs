use std::thread;
use std::time::Duration;

// Enum representing the possible light colors
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

// Controller struct manages multiple traffic lights
struct Controller {
    lights: Vec<TrafficLight>,
    active_index: usize  // index of the currently active (green) light
}

impl Controller {
    // Tick function simulates one step in time for all traffic lights
    fn tick(&mut self) {
        println!("--- TICK ---");

        // Print all lights and their states
        for (i, light) in self.lights.iter().enumerate() {
            let marker = if i == self.active_index { "<-- active" } else { "" };
            println!("Light {}: {:?} {}", i, light.state, marker);
        }

        // Handle state transition for the active light
        // We make a copy of active_index so we can borrow the corresponding light mutably, ends the borrow immediately and active_index is independent of self
        // This avoids the immutable + mutable borrow conflict that Rust enforces  
        let active_index = self.active_index;
        let active_light = &mut self.lights[active_index];

         // Simulate the duration the light stays in its current state
         thread::sleep(active_light.duration);

         // Update the state of the active light
         active_light.state = next_state(&active_light.state);

         // Update how long the new state should last
         active_light.duration = transition_time(&active_light.state);

        // If the light turned red, move to the next light
        // Ensures only one light is "active" at a time
         if active_light.state == LightColor::Red {
                self.active_index = (self.active_index +1) % self.lights.len();
            }
    }
}

// The function only needs to inspect the state to determine the next one
fn next_state(current: &LightColor) -> LightColor {
    match current {
        LightColor::Red => LightColor::Green,
        LightColor::Green => LightColor::Yellow,
        LightColor::Yellow => LightColor::Red,
    }
}

// Determines how long each light should stay in its state
fn transition_time(current: &LightColor) -> Duration {
    match current {
        LightColor::Red => Duration::from_secs(3),
        LightColor::Green => Duration::from_secs(3),
        LightColor::Yellow => Duration::from_secs(1),
    }
}


fn main() {
    // Create controller
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

    // Infinite loop to simulate traffic light operation
    loop {
        controller.tick();
     }
}
