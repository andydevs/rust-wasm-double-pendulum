use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

    // Get document
    let document = web_sys::window()
        .ok_or(JsValue::from("Could not find window!"))?
        .document()
        .ok_or(JsValue::from("Could not find document!"))?;
    console_log!("Get document");

    // Create and attach canvas element
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_attribute("width", "1200")?;
    canvas.set_attribute("height", "900")?;
    document
        .body()
        .ok_or(JsValue::from("Could not find document body!"))?
        .append_child(&canvas)?;
    console_log!("Create canvas");

    // Get canvas rendering context
    let ctx2d = canvas
        .get_context("2d")?
        .ok_or(JsValue::from("Could not create 2D drawing context!"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("Get render context");

    // Draw something
    ctx2d.rect(20.0, 20.0, 100.0, 100.0);
    ctx2d.fill();
    console_log!("Draw a rectangle");

    Ok(())
}
