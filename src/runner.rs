use wasm_bindgen::JsValue;

use crate::{
    anim::{AnimationLoopRunner, FrameCtx, WindowCtx},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

/// Runner for managing a simulation with rendering and updates in an animation loop.
///
/// This struct coordinates between a simulation and the browser's animation frame
/// callback, ensuring that the simulation is rendered and updated on each frame.
///
/// # Type Parameters
/// * `S` - The simulation type that implements the `Simulation` trait.
pub struct SimulationRunner<S: Simulation + 'static> {
    window: WindowCtx,
    sim: S,
}

impl<S: Simulation + 'static> SimulationRunner<S> {
    /// Creates a new SimulationRunner with the given simulation and window context.
    ///
    /// # Arguments
    /// * `state` - The initial simulation state.
    /// * `window` - The window and canvas context for rendering.
    pub fn new(state: S, window: WindowCtx) -> Self {
        Self { window, sim: state }
    }

    /// Starts the simulation loop.
    ///
    /// This method begins the animation loop using `requestAnimationFrame`.
    /// On each frame, the simulation is rendered and then updated in sequence.
    ///
    /// # Errors
    /// Returns a `JsValue` error if the animation frame request fails.
    pub fn run(mut self) -> Result<(), JsValue> {
        AnimationLoopRunner::new(move |frame: &FrameCtx| {
            // Render sim
            let render = RenderCtx {
                window: &self.window,
                frame,
            };
            self.sim.render(&render);

            // Update sim
            let update = UpdateCtx { frame };
            self.sim.update(&update);
        })
        .run()
    }
}
