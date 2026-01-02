# Traffic Light State Machine (Rust)

This project is a simple traffic light simulator written in Rust, designed to demonstrate
embedded-style state machine logic and real-time timing behavior.

## Overview

The program models a traffic light using:
- An enum (`LightColor`) to represent system states
- Pure functions for state transitions
- Timing logic using `std::time::Duration`
- A continuous control loop similar to embedded firmware main loops

## Why This Project

This project was created to practice:
- Rust enums and pattern matching
- Deterministic state transitions
- Separation of state logic and timing logic
- Concepts commonly used in embedded and real-time systems

## Current Features

- Red → Green → Yellow → Red state cycle
- Configurable timing per state
- Clear, readable state-machine implementation

## Planned Enhancements

- Multiple coordinated traffic lights (intersection logic)
- Pedestrian button / asynchronous events
- Parameterized timing
- Embedded hardware output (LEDs / GPIO) – optional

## How to Run

```bash
cargo run
