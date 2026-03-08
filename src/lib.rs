#[macro_use]
mod macros;
mod anim;
mod draw;
mod pendulum;
mod runner;
mod sim;
mod window;

use crate::pendulum::Pendulum;
use crate::runner::SimulationRunner;
use crate::window::WindowCtx;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

/// Entry point for the WebAssembly module.
///
/// Initializes the double pendulum simulation, sets up the canvas and rendering context,
/// and starts the main simulation loop.
///
/// # Errors
/// Returns a `JsValue` error if canvas initialization or the simulation loop fails.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

    // Get window context
    let window = WindowCtx::from_canvas_id("render-canvas")?;

    // Initial state
    let state = Pendulum::new(2.0, PI / 4.0, 0.0);

    // Run simulation
    SimulationRunner::new(state, window).run()
}
