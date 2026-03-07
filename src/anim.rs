use std::f64::consts::TAU;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const MILLIS_PER_SEC: f64 = 1000.0;

#[allow(dead_code)]
/// Context for a single animation frame,
/// containing the frame number, delta time, and timestamp.
pub struct FrameCtx {
    pub frame: u32,
    pub dt: f64,
    pub ts: f64,
}

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

/// A runner for managing an animation loop using the browser's
/// requestAnimationFrame.
///
/// This struct encapsulates the logic for running a continuous
/// animation loop, tracking frame counts, timestamps, and delta times.
/// It takes a callback function that is called for each frame with
/// a `FrameCtx` containing frame information.
pub struct AnimationLoopRunner<F: FnMut(&FrameCtx) + 'static> {
    frame_count: u32,
    last_ts: Option<f64>,
    callback: F,
}

impl<F: FnMut(&FrameCtx) + 'static> AnimationLoopRunner<F> {
    /// Creates a new AnimationLoopRunner with the given callback function.
    pub fn new(callback: F) -> Self {
        Self {
            frame_count: 0,
            last_ts: None,
            callback,
        }
    }

    /// Starts the animation loop. Continually
    /// requests new animation frame from browser
    pub fn run(mut self) -> Result<(), JsValue> {
        // Helper type for callback pointer.
        // Shared Ownership, interior mutability
        // And the ability to be undefined at runtime.
        type CallbackPtr = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

        // Create two pointers for underlying callback. One to store inside
        // callback to be recalled and one for outside to call the first time
        let inner_ptr: CallbackPtr = Rc::new(RefCell::new(None));
        let outer_ptr: CallbackPtr = Rc::clone(&inner_ptr);

        // Define the callback in one of the pointers, moving the other one
        *outer_ptr.borrow_mut() = Some(Closure::new(move |ts_ms| {
            // Initialize current frame (time step, dt, and frame count)
            let ts = ts_ms / MILLIS_PER_SEC;
            let dt = ts - self.last_ts.unwrap_or(ts);
            let frame = FrameCtx {
                frame: self.frame_count,
                dt,
                ts,
            };

            // Call contained callback for frame
            (self.callback)(&frame);

            // Update frame count
            self.frame_count += 1;
            self.last_ts = Some(frame.ts);

            // Request next animation frame
            window()
                .unwrap()
                .request_animation_frame(
                    inner_ptr
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        }));

        // Request the first animation frame with outer pointer
        window().unwrap().request_animation_frame(
            outer_ptr
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )?;
        Ok(())
    }
}
