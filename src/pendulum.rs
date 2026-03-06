use crate::sim::{RenderCtx, Simulation, UpdateCtx};

/// Acceleration due to gravity on Earth
const GRAVITY: f64 = 9.81;

/// A conversion between meters irl to pixels in screen space
/// I.e. how many pixels long represents a meter.
const METERS_TO_PIXELS: f64 = 200.0;

/// Represents a single pendulum in the simulation.
///
/// The pendulum has a fixed length and damping coefficient, and
/// tracks its current angle (theta) and angular velocity (omega).
pub struct Pendulum {
    pub length: f64,
    pub damping: f64,
    theta: f64,
    omega: f64,
}

impl Pendulum {
    /// Creates a new Pendulum with the specified length,
    /// damping, initial angle, and initial angular velocity.
    pub fn new(length: f64, damping: f64, theta_init: f64, omega_init: f64) -> Self {
        Self {
            length,
            damping,
            theta: theta_init,
            omega: omega_init,
        }
    }
}

impl Simulation for Pendulum {
    /// Renders the pendulum on the provided render context.
    ///
    /// Draws the pivot point, the rod, and the bob of the pendulum on the canvas.
    fn render(&self, render: &RenderCtx) {
        // Pivot
        let x0 = (render.window.canvas.width() as f64) / 2.0;
        let y0 = (render.window.canvas.height() as f64) * (1.0 / 4.0);

        // End
        let (s, c) = self.theta.sin_cos();
        let x1 = x0 + self.length * METERS_TO_PIXELS * s;
        let y1 = y0 + self.length * METERS_TO_PIXELS * c;

        render.window.clear();
        render.window.line(x0, y0, x1, y1, "#ffff00");
        render.window.circle(x0, y0, 5.0, "#ffffff");
        render.window.circle(x1, y1, 10.0, "#00aaff");
    }

    /// Updates the pendulum's state based on the update context.
    ///
    /// Applies the physics equations to update
    /// the angle and angular velocity for the next frame.
    ///
    /// dtheta/dt = omega
    /// domega/dt = -(gravity contribution) - dampening
    fn update(&mut self, update: &UpdateCtx) {
        let dt = update.frame.dt;
        let gravity = -self.theta.sin() * GRAVITY / self.length;
        let damping = -self.damping * self.omega;
        self.theta += self.omega * dt;
        self.omega += (gravity + damping) * dt;
    }
}
