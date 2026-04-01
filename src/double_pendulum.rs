//! Double pendulum physics simulation.
//!
//! Implements the equations of motion for a planar double pendulum using
//! Lagrangian mechanics and forward Euler numerical integration.

use crate::{
    consts::{GRAVITY, METERS_TO_PIXELS, MILLIS_PER_SEC},
    draw::{FilledCircle, Line, Style},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

/// A double pendulum: two rigid rods connected end-to-end, each free to rotate.
///
/// Angles (`theta_1`, `theta_2`) are measured from the downward vertical in radians.
/// Angular velocities (`omega_1`, `omega_2`) are in radians per second.
/// Physics are integrated with a forward Euler step each frame.
pub struct DoublePendulum {
    /// Length of the first (upper) rod, in metres.
    pub length_1: f64,
    /// Length of the second (lower) rod, in metres.
    pub length_2: f64,
    /// Angle of the first joint from the downward vertical, in radians.
    theta_1: f64,
    /// Angular velocity of the first joint, in radians per second.
    omega_1: f64,
    /// Angle of the second joint relative to the first rod, in radians.
    theta_2: f64,
    /// Angular velocity of the second joint, in radians per second.
    omega_2: f64,
}

impl DoublePendulum {
    /// Creates a new `DoublePendulum` with the given rod lengths and initial conditions.
    ///
    /// # Arguments
    /// * `length_0` - Length of the first (upper) rod in metres.
    /// * `length_1` - Length of the second (lower) rod in metres.
    /// * `theta_0` - Initial angle of the first joint from the downward vertical, in radians.
    /// * `omega_0` - Initial angular velocity of the first joint, in radians per second.
    /// * `theta_1` - Initial angle of the second joint relative to the first rod, in radians.
    /// * `omega_1` - Initial angular velocity of the second joint, in radians per second.
    pub fn new(
        length_0: f64,
        length_1: f64,
        theta_0: f64,
        omega_0: f64,
        theta_1: f64,
        omega_1: f64,
    ) -> Self {
        Self {
            length_1: length_0,
            length_2: length_1,
            theta_1: theta_0,
            omega_1: omega_0,
            theta_2: theta_1,
            omega_2: omega_1,
        }
    }
}

impl Simulation for DoublePendulum {
    /// Renders the double pendulum to the canvas.
    ///
    /// Clears the canvas and draws:
    /// - A white circle at the fixed pivot point.
    /// - A yellow rod from the pivot to the first joint, with a green circle at the joint.
    /// - A magenta rod from the first joint to the second bob, with a cyan circle at the bob.
    fn render(&self, render: &RenderCtx) {
        // Pivot
        let x_0 = (render.window.canvas.width() as f64) / 2.0;
        let y_0 = (render.window.canvas.height() as f64) * (1.0 / 4.0);

        // First Node
        let (s, c) = self.theta_1.sin_cos();
        let x_1 = x_0 + self.length_1 * METERS_TO_PIXELS * s;
        let y_1 = y_0 + self.length_1 * METERS_TO_PIXELS * c;

        // Second Node
        let (s, c) = (self.theta_1 + self.theta_2).sin_cos();
        let x_2 = x_1 + self.length_2 * METERS_TO_PIXELS * s;
        let y_2 = y_1 + self.length_2 * METERS_TO_PIXELS * c;

        // Render
        render.clear();
        render.draw(&Line(x_0, y_0, x_1, y_1).styled().stroke("#ffff00".into()));
        render.draw(&Line(x_1, y_1, x_2, y_2).styled().stroke("#ff00ff".into()));
        render.draw(
            &FilledCircle((x_0, y_0), 5.0)
                .styled()
                .fill("#ffffff".into()),
        );
        render.draw(
            &FilledCircle((x_1, y_1), 10.0)
                .styled()
                .fill("#00ff00".into()),
        );
        render.draw(
            &FilledCircle((x_2, y_2), 10.0)
                .styled()
                .fill("#00aaff".into()),
        );
    }

    /// Advances the simulation by one frame using forward Euler integration.
    ///
    /// Computes the angular accelerations `alpha_1` and `alpha_2` from the
    /// Lagrangian equations of motion for a double pendulum, then integrates:
    ///
    /// ```text
    /// dtheta/dt = omega
    /// domega/dt = alpha
    /// ```
    ///
    /// The shared denominator term is `2 - cos²(theta_1 - theta_2)`, which arises
    /// from the coupling between the two rods.
    fn update(&mut self, update: &UpdateCtx) {
        // time delta
        let dt = update.frame.delta / MILLIS_PER_SEC;

        // Acceleration terms
        let sin_1 = self.theta_1.sin();
        let cos_1 = self.theta_1.cos();
        let sin_diff = (self.theta_1 - self.theta_2).sin();
        let cos_diff = (self.theta_1 - self.theta_2).cos();
        let denom = 2.0 - cos_diff * cos_diff;
        let omega_1_sq = self.omega_1 * self.omega_1;
        let omega_2_sq = self.omega_2 * self.omega_2;
        let alpha_1 = (-GRAVITY * (2.0 * sin_1 - sin_diff * cos_diff)
            - omega_2_sq * self.length_2 * sin_diff
            - omega_1_sq * self.length_1 * sin_1 * cos_diff)
            / (self.length_1 * denom);
        let alpha_2 = (2.0 * sin_diff * (omega_1_sq * self.length_1 * cos_diff + GRAVITY * cos_1)
            + omega_2_sq * self.length_2 * sin_diff * cos_diff)
            / (self.length_2 * denom);

        // Theta and omega updates
        self.theta_1 += self.omega_1 * dt;
        self.omega_1 += alpha_1 * dt;
        self.theta_2 += self.omega_2 * dt;
        self.omega_2 += alpha_2 * dt;
    }
}
