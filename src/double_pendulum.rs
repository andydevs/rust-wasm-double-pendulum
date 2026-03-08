use crate::{
    consts::{GRAVITY, METERS_TO_PIXELS},
    draw::{FilledCircle, Line, Style},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

/// Represents a single pendulum in the simulation.
///
/// The pendulum has a fixed length and damping coefficient, and
/// tracks its current angle (theta) and angular velocity (omega).
pub struct DoublePendulum {
    pub length_1: f64,
    pub length_2: f64,
    theta_1: f64,
    omega_1: f64,
    theta_2: f64,
    omega_2: f64,
}

impl DoublePendulum {
    /// Creates a new DoublePendulum with the specified length,
    /// damping, initial angle, and initial angular velocity.
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
    /// Renders the pendulum on the provided render context.
    ///
    /// Draws the pivot point, the rod, and the bob of the pendulum on the canvas.
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

    /// Updates the pendulum's state based on the update context.
    ///
    /// Applies the physics equations to update
    /// the angle and angular velocity for the next frame.
    ///
    /// dtheta/dt = omega
    /// domega/dt = -(gravity contribution) - dampening
    fn update(&mut self, update: &UpdateCtx) {
        // time delta
        let dt = update.frame.dt;

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
