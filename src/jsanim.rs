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

    /// Clears the entire canvas.
    pub fn clear(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, width, height);
    }

    /// Draws a filled circle at the specified
    /// position with the given radius and color.
    pub fn circle(&self, x: f64, y: f64, r: f64, color: &str) {
        self.ctx.save();
        self.ctx.set_fill_style_str(color);
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, TAU).unwrap();
        self.ctx.fill();
        self.ctx.restore();
    }

    /// Draws a line from (x0, y0) to (x1, y1) with the specified color.
    pub fn line(&self, x0: f64, y0: f64, x1: f64, y1: f64, color: &str) {
        self.ctx.save();
        self.ctx.set_stroke_style_str(color);
        self.ctx.begin_path();
        self.ctx.move_to(x0, y0);
        self.ctx.line_to(x1, y1);
        self.ctx.stroke();
        self.ctx.restore();
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
