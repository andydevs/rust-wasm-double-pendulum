pub type Vector2D = (f64, f64);

pub trait DynamicObject {
    fn get_velocity(&self) -> Vector2D;
    fn get_position(&self) -> Vector2D;
    fn set_position(&mut self, position: Vector2D);

    fn update(&mut self, dt: f64) {
        // Update position
        let pos = self.get_position();
        let vel = self.get_velocity();
        let dx = (vel.0 * dt, vel.1 * dt);
        let new_pos = (pos.0 + dx.0, pos.1 + dx.1);
        self.set_position(new_pos);
    }
}
