use crate::anim::run_request_animation_frame_loop;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys::Date};

pub struct RenderCtx {
    pub width: u32,
    pub height: u32,
    pub ctx2d: CanvasRenderingContext2d,
}

pub struct FrameCtx {
    pub number: u32,
    pub delta_t: f64,
}

pub struct SimulationContext<S> {
    pub render: RenderCtx,
    pub frame: FrameCtx,
    pub sim_state: S,
}

pub type SimCtx<S> = SimulationContext<S>;

pub const MS_PER_SEC: f64 = 1000.0;

pub fn run_simulation_loop<S: 'static, F: FnMut(&mut SimulationContext<S>) + 'static>(
    canvas: HtmlCanvasElement,
    ctx2d: CanvasRenderingContext2d,
    initial_state: S,
    mut render_update: F,
) -> Result<(), JsValue> {
    let mut ctx = SimulationContext {
        render: RenderCtx {
            width: canvas.width(),
            height: canvas.height(),
            ctx2d,
        },
        frame: FrameCtx {
            number: 0,
            delta_t: 0.0,
        },
        sim_state: initial_state,
    };

    let mut time_end = Date::now();
    let mut time_start = Date::now();
    run_request_animation_frame_loop(move || {
        time_end = Date::now();

        render_update(&mut ctx);

        ctx.frame.number += 1;
        ctx.frame.delta_t = (time_end - time_start) / MS_PER_SEC;

        time_start = Date::now();
    })
}
