mod anim;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use crate::anim::animation_frame_loop;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

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
    let ctx2d = canvas
        .get_context("2d")?
        .ok_or(JsValue::from("Could not create 2D drawing context!"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("Get render context");

    let mut x = 2.0;
    let mut y = 1.0;
    animation_frame_loop(move || {
        (&ctx2d).clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        (&ctx2d).fill_rect(x, y, 10.0, 10.0);

        x += 0.2;
        y += 0.8;
    })
}
