#[macro_use]
mod macros;
mod jsanim;
mod runner;
mod sim;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use crate::{
    jsanim::WindowCtx,
    runner::SimulationRunner,
    sim::{RenderCtx, Simulation, UpdateCtx},
};

pub struct State {
    pub x: (f64, f64),
    pub v: (f64, f64),
    a: (f64, f64),
}

impl State {
    pub fn initial(x: (f64, f64), v: (f64, f64), a: (f64, f64)) -> Self {
        Self { x, v, a }
    }
}

impl Simulation for State {
    fn render(&self, render: &RenderCtx) {
        render.window.clear();
        render.window.circle(self.x.0, self.x.1, 10.0, "#00aabb");
    }

    fn update(&mut self, update: &UpdateCtx) {
        self.x.0 += self.v.0 * update.frame.dt;
        self.x.1 += self.v.1 * update.frame.dt;
        self.v.0 += self.a.0 * update.frame.dt;
        self.v.1 += self.a.1 * update.frame.dt;
    }
}

fn get_window_ctx() -> Result<WindowCtx, JsValue> {
    // Get canvas
    let canvas = window()
        .ok_or(JsValue::from("Unable to get browser window!"))?
        .document()
        .ok_or(JsValue::from("Unable to get document!"))?
        .get_element_by_id("render-canvas")
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
    let window = get_window_ctx()?;

    // Initial state
    let state = State::initial(
        (15.0, window.canvas.height() as f64 - 15.0),
        (0.4, -1.0),
        (0.0, 0.001),
    );

    // Run simulation
    SimulationRunner::new(state, window).run()
}
