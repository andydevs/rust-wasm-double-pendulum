use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use crate::draw::Draw;

/// Context for the window and canvas,
/// providing utilities for drawing on the canvas.
pub struct WindowCtx {
    pub ctx: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

impl WindowCtx {
    /// Creates a new WindowCtx from a canvas element with the given ID.
    pub fn from_canvas_id(id: &str) -> Result<Self, JsValue> {
        // Get canvas
        let canvas = window()
            .ok_or(JsValue::from("Unable to get browser window!"))?
            .document()
            .ok_or(JsValue::from("Unable to get document!"))?
            .get_element_by_id(id)
            .ok_or(JsValue::from("Couldn't find #render-canvas!"))?
            .dyn_into::<HtmlCanvasElement>()?;
        console_log!("Get canvas for id {}", id);

        // Get canvas rendering context
        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from("Could not create 2D drawing context!"))?
            .dyn_into::<CanvasRenderingContext2d>()?;
        console_log!("Get 2d render context");

        // Return window
        let window = Self::new(canvas, ctx);
        Ok(window)
    }

    /// Creates a new WindowCtx with the given canvas and rendering context.
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx, canvas }
    }

    /// Double-dispatch draw method
    pub fn draw(&self, d: &dyn Draw) {
        d.draw(self);
    }

    /// Clears the entire canvas.
    pub fn clear(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, width, height);
    }
}
