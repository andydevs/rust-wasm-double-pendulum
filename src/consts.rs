//! Shared physical and rendering constants used throughout the simulation.

/// Acceleration due to gravity on Earth, in m/s².
pub const GRAVITY: f64 = 9.81;

/// Scale factor converting metres to canvas pixels (pixels per metre).
///
/// A pendulum rod that is 1 metre long will be drawn 200 pixels long on screen.
pub const METERS_TO_PIXELS: f64 = 200.0;

/// Number of milliseconds in one second.
///
/// Used to convert the frame delta time from milliseconds (as provided by
/// `requestAnimationFrame`) into seconds for physics integration.
pub const MILLIS_PER_SEC: f64 = 1000.0;
