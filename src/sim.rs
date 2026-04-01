//! Core simulation traits and per-frame context types.
//!
//! Defines the [`Simulation`] trait that all physics simulations must implement,
//! as well as the [`RenderCtx`] and [`UpdateCtx`] types that carry per-frame
//! information into each `render` and `update` call respectively.

use crate::{draw::Draw, window::WindowCtx};
use wasm_raf_handler::FrameCtx;

/// Context for rendering a frame of the simulation.
///
/// Provides access to the window and canvas context as well as frame-specific information.
/// Passed to the `render` method of simulations to provide drawing capabilities.
#[allow(dead_code)]
pub struct RenderCtx<'s> {
    /// Canvas and window context used to issue draw calls.
    pub window: &'s WindowCtx,
    /// Frame timing information (timestamp, delta) for the current animation frame.
    pub frame: &'s FrameCtx,
}

impl<'s> RenderCtx<'s> {
    /// Clears the canvas.
    ///
    /// Fills the entire canvas with a blank state, removing any previously drawn content.
    pub fn clear(&self) {
        self.window.clear();
    }

    /// Draws a drawable object to the canvas.
    ///
    /// Takes any object implementing the `Draw` trait and renders it to the canvas.
    ///
    /// # Arguments
    /// * `d` - A trait object implementing `Draw` to be rendered.
    pub fn draw(&self, d: &dyn Draw) {
        self.window.draw(d);
    }
}

/// Context for updating the simulation.
///
/// Provides frame-specific information such as the current timestamp and delta time.
/// Passed to the `update` method of simulations to allow time-based updates.
#[allow(dead_code)]
pub struct UpdateCtx<'s> {
    /// Frame timing information (timestamp, delta) for the current animation frame.
    pub frame: &'s FrameCtx,
}

/// Trait for simulation objects that can be rendered and updated.
///
/// Implementations of this trait define how a simulation is rendered on each frame
/// and how the simulation state is updated based on elapsed time.
pub trait Simulation {
    /// Renders the current state of the simulation.
    ///
    /// This method is called on each animation frame to draw the simulation
    /// to the canvas.
    ///
    /// # Arguments
    /// * `render` - The rendering context containing the window and frame information.
    fn render(&self, render: &RenderCtx);

    /// Updates the simulation state.
    ///
    /// This method is called on each animation frame to update the simulation
    /// based on the elapsed time since the last frame.
    ///
    /// # Arguments
    /// * `update` - The update context containing frame timing information.
    fn update(&mut self, update: &UpdateCtx);
}
