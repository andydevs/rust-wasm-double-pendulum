use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::window;

const MILLIS_PER_SEC: f64 = 1000.0;

#[allow(dead_code)]
/// Context for a single animation frame,
/// containing the frame number, delta time, and timestamp.
pub struct FrameCtx {
    pub frame: u32,
    pub dt: f64,
    pub ts: f64,
}
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
