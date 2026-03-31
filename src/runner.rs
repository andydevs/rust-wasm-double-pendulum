use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;
use wasm_raf_handler::{FrameCtx, RAFLoop};

use crate::{
    sim::{RenderCtx, Simulation, UpdateCtx},
    window::WindowCtx,
};

/// Runner for managing a simulation with rendering and updates in an animation loop.
///
/// This struct coordinates between a simulation and the browser's animation frame
/// callback, ensuring that the simulation is rendered and updated on each frame.
///
/// # Type Parameters
/// * `S` - The simulation type that implements the `Simulation` trait.
pub struct SimulationRunner<S: Simulation + 'static> {
    rafloop: Option<RAFLoop>,
    window: Rc<WindowCtx>,
    sim: Rc<RefCell<S>>,
}

impl<S: Simulation + 'static> SimulationRunner<S> {
    /// Creates a new SimulationRunner with the given simulation and window context.
    ///
    /// # Arguments
    /// * `state` - The initial simulation state.
    /// * `window` - The window and canvas context for rendering.
    pub fn new(state: S, window: WindowCtx) -> Self {
        Self {
            rafloop: None,
            window: Rc::new(window),
            sim: Rc::new(RefCell::new(state)),
        }
    }

    /// Starts the simulation loop.
    ///
    /// This method begins the animation loop using `requestAnimationFrame`.
    /// On each frame, the simulation is rendered and then updated in sequence.
    ///
    /// # Errors
    /// Returns a `JsValue` error if the animation frame request fails.
    pub fn run(&mut self) -> Result<(), JsValue> {
        console_log!("Create Rc clones");
        let inner_window = Rc::clone(&self.window);
        let inner_sim = Rc::clone(&self.sim);
        console_log!("Create RAFLoop");
        let rafloop = RAFLoop::new(move |frame: FrameCtx| {
            console_log!("Create old frame context");
            let old_frame_ctx = crate::anim::FrameCtx {
                frame: frame.frame_count,
                dt: frame.delta,
                ts: frame.timestamp,
            };

            {
                // Render sim
                console_log!("Render simulation");
                let render = RenderCtx {
                    window: &inner_window,
                    frame: &old_frame_ctx,
                };
                inner_sim.borrow().render(&render);
            };
            {
                // Update sim
                console_log!("Update simulation");
                let update = UpdateCtx {
                    frame: &old_frame_ctx,
                };
                inner_sim.borrow_mut().update(&update);
            };
        })?;
        console_log!("Set RAFLoop");
        self.rafloop = Some(rafloop);
        Ok(())
    }
}
