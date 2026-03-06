use wasm_bindgen::JsValue;

use crate::{
    jsanim::{AnimationLoopRunner, FrameCtx, WindowCtx},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

pub struct SimulationRunner<S: Simulation + 'static> {
    window: WindowCtx,
    sim: S,
}

impl<S: Simulation + 'static> SimulationRunner<S> {
    pub fn new(state: S, window: WindowCtx) -> Self {
        Self { window, sim: state }
    }

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
