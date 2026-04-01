//! Single pendulum physics simulation (currently unused).
//!
//! Kept as a reference implementation and for comparison against the
//! more complex [`crate::double_pendulum::DoublePendulum`].

use crate::{
    consts::{GRAVITY, METERS_TO_PIXELS, MILLIS_PER_SEC},
    draw::{FilledCircle, Line, Style},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

/// A simple single-rod pendulum.
///
/// The angle `theta` is measured from the downward vertical in radians, and
/// `omega` is the angular velocity in radians per second. Physics are integrated
/// with a forward Euler step each frame.
pub struct Pendulum {
    /// Length of the pendulum rod, in metres.
    pub length: f64,
    /// Current angle from the downward vertical, in radians.
    theta: f64,
    /// Current angular velocity, in radians per second.
    omega: f64,
}

impl Pendulum {
    /// Creates a new `Pendulum` with the given rod length and initial conditions.
    ///
    /// # Arguments
    /// * `length` - Length of the rod in metres.
    /// * `theta_init` - Initial angle from the downward vertical, in radians.
    /// * `omega_init` - Initial angular velocity, in radians per second.
    pub fn new(length: f64, theta_init: f64, omega_init: f64) -> Self {
        Self {
            length,
            theta: theta_init,
            omega: omega_init,
        }
    }
}

impl Simulation for Pendulum {
    /// Renders the pendulum to the canvas.
    ///
    /// Clears the canvas and draws a yellow rod from the fixed pivot to the bob,
    /// a white circle at the pivot, and a cyan circle at the bob.
    fn render(&self, render: &RenderCtx) {
        // Pivot
        let x0 = (render.window.canvas.width() as f64) / 2.0;
        let y0 = (render.window.canvas.height() as f64) * (1.0 / 4.0);

        // End
        let (s, c) = self.theta.sin_cos();
        let x1 = x0 + self.length * METERS_TO_PIXELS * s;
        let y1 = y0 + self.length * METERS_TO_PIXELS * c;

        // Render
        render.clear();
        render.draw(&Line(x0, y0, x1, y1).styled().stroke("#ffff00".into()));
        render.draw(&FilledCircle((x0, y0), 5.0).styled().fill("#ffffff".into()));
        render.draw(&FilledCircle((x1, y1), 10.0).styled().fill("#00aaff".into()));
    }

    /// Advances the simulation by one frame using forward Euler integration.
    ///
    /// Applies the small-angle–free equation of motion:
    ///
    /// ```text
    /// alpha = -sin(theta) * g / L
    /// dtheta/dt = omega
    /// domega/dt = alpha
    /// ```
    fn update(&mut self, update: &UpdateCtx) {
        let dt = update.frame.delta / MILLIS_PER_SEC;
        let gravity = -self.theta.sin() * GRAVITY / self.length;
        self.theta += self.omega * dt;
        self.omega += gravity * dt;
    }
}
