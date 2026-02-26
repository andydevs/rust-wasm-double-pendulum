use wasm_bindgen::JsValue;

use crate::{
    jsanim::{FrameCtx, WindowCtx, animation_frame_loop},
    sim::{RenderCtx, Simulation, UpdateCtx},
};

pub struct SimCtxState {
    frame_count: u32,
    end_timestamp: Option<f64>,
}

impl Default for SimCtxState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            end_timestamp: None,
        }
    }
}

pub struct SimulationRunner<S: Simulation + 'static> {
    ctx: SimCtxState,
    window: WindowCtx,
    sim: S,
}

impl<S: Simulation + 'static> SimulationRunner<S> {
    pub fn new(state: S, window: WindowCtx) -> Self {
        Self {
            ctx: SimCtxState::default(),
            window,
            sim: state,
        }
    }

    pub fn run(mut self) -> Result<(), JsValue> {
        animation_frame_loop(move |timestamp| {
            // Get frame information
            let frame = self.frame_start(timestamp);

            // Render sim
            let render = RenderCtx {
                window: &self.window,
                frame: &frame,
            };
            self.sim.render(&render);

            // Update sim
            let update = UpdateCtx { frame: &frame };
            self.sim.update(&update);

            // Update context
            self.frame_end(timestamp);
        })
    }

    fn frame_end(&mut self, timestamp: f64) {
        self.ctx.frame_count += 1;
        self.ctx.end_timestamp = Some(timestamp);
    }

    fn frame_start(&self, timestamp: f64) -> FrameCtx {
        let dt = timestamp - self.ctx.end_timestamp.unwrap_or(timestamp);
        FrameCtx {
            frame: self.ctx.frame_count,
            dt,
            timestamp,
        }
    }
}
