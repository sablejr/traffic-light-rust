# Traffic Light Controller (Rust UPDATED FOR 2.0)

A simple traffic light simulation implemented as a state machine in Rust.

## Design
- Each `TrafficLight` owns its own state and duration
- A central `Controller` owns all lights
- Only the active light is allowed to change state
- When a light completes a cycle (returns to Red), control moves to the next light

## Concepts Demonstrated
- Enums and pattern matching
- Ownership vs borrowing (`&mut`)
- Controller pattern
- Safe state transitions
- Avoiding shared mutable state

## How to Run
```bash
cargo run
