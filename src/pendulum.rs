use crate::sim::{RenderCtx, Simulation, UpdateCtx};

pub struct Pendulum {
    pub x: (f64, f64),
    pub v: (f64, f64),
    a: (f64, f64),
}

impl Pendulum {
    pub fn initial(x: (f64, f64), v: (f64, f64), a: (f64, f64)) -> Self {
        Self { x, v, a }
    }
}

impl Simulation for Pendulum {
    fn render(&self, render: &RenderCtx) {
        render.window.clear();
        render.window.circle(self.x.0, self.x.1, 10.0, "#00aabb");
    }

    fn update(&mut self, update: &UpdateCtx) {
        self.x.0 += self.v.0 * update.frame.dt;
        self.x.1 += self.v.1 * update.frame.dt;
        self.v.0 += self.a.0 * update.frame.dt;
        self.v.1 += self.a.1 * update.frame.dt;
    }
}
