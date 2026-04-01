//! Browser window and HTML5 canvas context management.
//!
//! [`WindowCtx`] bundles the `HtmlCanvasElement` and its `CanvasRenderingContext2d`
//! together and provides convenience methods used by the rendering pipeline.

use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use crate::draw::Draw;

/// Bundles an HTML canvas element with its 2D rendering context.
///
/// Obtained via [`WindowCtx::from_canvas_id`] and then shared across the
/// rendering pipeline through `Rc<WindowCtx>`.
pub struct WindowCtx {
    /// The 2D rendering context used to issue canvas draw calls.
    pub ctx: CanvasRenderingContext2d,
    /// The underlying HTML canvas element (used to query width/height).
    pub canvas: HtmlCanvasElement,
}

impl WindowCtx {
    /// Looks up a canvas element by DOM ID and creates a `WindowCtx` from it.
    ///
    /// # Errors
    /// Returns a `JsValue` error if the browser window, document, or canvas element
    /// cannot be retrieved, or if acquiring the 2D rendering context fails.
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

    /// Creates a `WindowCtx` directly from an existing canvas and rendering context.
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx, canvas }
    }

    /// Dispatches a draw call to the given [`Draw`] implementor.
    ///
    /// This is a thin double-dispatch wrapper: the drawable receives `self`
    /// (the canvas context) and issues the appropriate canvas API calls.
    pub fn draw(&self, d: &dyn Draw) {
        d.draw(self);
    }

    /// Clears the entire canvas by calling `clearRect` over its full bounds.
    pub fn clear(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, width, height);
    }
}
