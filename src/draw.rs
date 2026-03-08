use std::f64::consts::TAU;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Context for the window and canvas,
/// providing utilities for drawing on the canvas.
pub struct WindowCtx {
    pub ctx: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

/// A wrapper for applying styling to drawable objects.
///
/// This struct applies fill and stroke styles to any drawable object
/// without modifying the original. It uses the canvas context's save/restore
/// mechanism to ensure styles are properly scoped and don't affect other drawings.
///
/// # Type Parameters
/// * `C` - The contained drawable type.
pub struct Style<C: Draw> {
    pub contained: C,
    pub fill_style: Option<String>,
    pub stroke_style: Option<String>,
}

impl<C: Draw> Draw for Style<C> {
    /// Draws the contained drawable with the specified fill and stroke styles applied.
    ///
    /// This method saves the current canvas state, applies the configured styles,
    /// draws the contained object, and then restores the canvas state.
    fn draw(&self, window: &WindowCtx) {
        window.ctx.save();
        if let Some(fill) = &self.fill_style {
            window.ctx.set_fill_style_str(&fill);
        }
        if let Some(stroke) = &self.stroke_style {
            window.ctx.set_stroke_style_str(&stroke);
        }
        self.contained.draw(window);
        window.ctx.restore();
    }
}

/// A filled circle drawable.
///
/// Represents a circle defined by its center coordinates and radius.
/// The circle is filled using the current fill style of the canvas context.
///
/// # Fields
/// * `0` - A tuple `(x, y)` representing the center coordinates.
/// * `1` - The radius of the circle.
pub struct FilledCircle(pub (f64, f64), pub f64);

impl Draw for FilledCircle {
    /// Draws a filled circle at the specified center and radius.
    ///
    /// The circle is drawn using the current fill style of the canvas context.
    fn draw(&self, window: &WindowCtx) {
        let Self((x, y), r) = self;
        window.ctx.begin_path();
        window.ctx.arc(*x, *y, *r, 0.0, TAU).unwrap();
        window.ctx.fill();
    }
}

/// A line segment drawable.
///
/// Represents a line from one point to another.
/// The line is stroked using the current stroke style of the canvas context.
///
/// # Fields
/// * `0` - The x-coordinate of the starting point.
/// * `1` - The y-coordinate of the starting point.
/// * `2` - The x-coordinate of the ending point.
/// * `3` - The y-coordinate of the ending point.
pub struct Line(pub f64, pub f64, pub f64, pub f64);

impl Draw for Line {
    /// Draws a line segment between the two specified points.
    ///
    /// The line is stroked using the current stroke style of the canvas context.
    fn draw(&self, window: &WindowCtx) {
        let Self(x0, y0, x1, y1) = self;
        window.ctx.begin_path();
        window.ctx.move_to(*x0, *y0);
        window.ctx.line_to(*x1, *y1);
        window.ctx.stroke();
    }
}

/// Trait for objects that can be drawn on a canvas.
///
/// Types implementing this trait can be rendered to a canvas by calling
/// the `draw` method with a `WindowCtx` providing access to the canvas
/// rendering context.
pub trait Draw {
    fn draw(&self, window: &WindowCtx);
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
