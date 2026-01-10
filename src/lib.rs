mod anim;
mod dynamics;
mod sim;

use crate::{
    dynamics::{DynamicObject, Vector2D},
    sim::{SimCtx, run_simulation_loop},
};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

struct AnimState {
    position: Vector2D,
    velocity: Vector2D,
}

impl DynamicObject for AnimState {
    fn get_position(&self) -> Vector2D {
        self.position
    }

    fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    fn get_velocity(&self) -> Vector2D {
        self.velocity
    }
}

fn loop_fn(ctx: &mut SimCtx<AnimState>) {
    let state = &mut ctx.sim_state;

    // Draw something
    {
        let ctx2d = &ctx.render.ctx2d;
        ctx2d.clear_rect(0.0, 0.0, ctx.render.width as f64, ctx.render.height as f64);
        ctx2d.fill_rect(state.position.0, state.position.1, 100.0, 100.0);
    }

    // Update state
    state.update(ctx.frame.delta_t);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

    // Get canvas
    let canvas = window()
        .ok_or::<JsValue>("Unable to get browser window!".into())?
        .document()
        .ok_or::<JsValue>("Unable to get document!".into())?
        .get_element_by_id("render-canvas")
        .ok_or::<JsValue>("Couldn't find #render-canvas!".into())?
        .dyn_into::<HtmlCanvasElement>()?;
    console_log!("Get canvas");

    // Get canvas rendering context
    let ctx2d = canvas
        .get_context("2d")?
        .ok_or(JsValue::from("Could not create 2D drawing context!"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("Get render context");

    // Start an animation loop
    let initial_state = AnimState {
        position: (20.0, 20.0),
        velocity: (100.0, 50.0),
    };
    run_simulation_loop(canvas, ctx2d, initial_state, loop_fn)
}
