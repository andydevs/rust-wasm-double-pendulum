#[macro_use]
mod macros;
mod jsanim;
mod pendulum;
mod runner;
mod sim;

use crate::{jsanim::WindowCtx, pendulum::Pendulum, runner::SimulationRunner};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

fn get_window_ctx(id: &str) -> Result<WindowCtx, JsValue> {
    // Get canvas
    let canvas = window()
        .ok_or(JsValue::from("Unable to get browser window!"))?
        .document()
        .ok_or(JsValue::from("Unable to get document!"))?
        .get_element_by_id(id)
        .ok_or(JsValue::from("Couldn't find #render-canvas!"))?
        .dyn_into::<HtmlCanvasElement>()?;
    console_log!("Get canvas");

    // Get canvas rendering context
    let ctx = canvas
        .get_context("2d")?
        .ok_or(JsValue::from("Could not create 2D drawing context!"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("Get render context");

    // Return window
    let window = WindowCtx::new(canvas, ctx);
    Ok(window)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

    // Get window context
    let window = get_window_ctx("render-canvas")?;

    // Initial state
    let state = Pendulum::initial(
        (15.0, window.canvas.height() as f64 - 15.0),
        (0.4, -1.0),
        (0.0, 0.001),
    );

    // Run simulation
    SimulationRunner::new(state, window).run()
}
