use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

struct AnimationContext<S> {
    canvas_context: CanvasRenderingContext2d,
    frame_number: i32,
    anim_state: S,
}

fn animation_loop<S: 'static, F: FnMut(&mut AnimationContext<S>) + 'static>(
    canvas_context: CanvasRenderingContext2d,
    mut f: F,
    initial_state: S,
) -> Result<(), JsValue> {
    let frame_fn0 = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let frame_fn1 = frame_fn0.clone();
    let mut ctx = AnimationContext {
        anim_state: initial_state,
        canvas_context: canvas_context,
        frame_number: 0,
    };
    *frame_fn1.borrow_mut() = Some(Closure::new(move || {
        f(&mut ctx);
        ctx.frame_number += 1;
        let _ = window().unwrap().request_animation_frame(
            frame_fn0
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        );
    }));
    window().unwrap().request_animation_frame(
        frame_fn1
            .borrow()
            .as_ref()
            .unwrap()
            .as_ref()
            .unchecked_ref(),
    )?;
    Ok(())
}

struct AnimState {
    position: (f64, f64),
    velocity: (f64, f64),
}

fn loop_fn(ctx: &mut AnimationContext<AnimState>) {
    let ctx2d = &ctx.canvas_context;
    let state = &mut ctx.anim_state;

    // Draw something
    ctx2d.clear_rect(0.0, 0.0, 1200.0, 900.0);
    ctx2d.rect(state.position.0, state.position.1, 100.0, 100.0);
    ctx2d.fill();

    // Update state
    state.position = (
        state.position.0 + state.velocity.0,
        state.position.1 + state.velocity.1,
    );
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log!("main() function in lib.rs called");

    // Get canvas
    let canvas = window()
        .ok_or::<JsValue>("Unable to get browser window!".into())?
        .document()
        .ok_or::<JsValue>("Unable to get document!".into())?
        .get_element_by_id("render-canvas")
        .ok_or::<JsValue>("Couldn't find #render-canvas!".into())?
        .dyn_into::<HtmlCanvasElement>()?;
    console_log!("Get canvas");

    // Get canvas rendering context
    let ctx2d = canvas
        .get_context("2d")?
        .ok_or(JsValue::from("Could not create 2D drawing context!"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("Get render context");

    // Start an animation loop
    let initial_state = AnimState {
        position: (20.0, 20.0),
        velocity: (1.0, 0.0),
    };
    animation_loop(ctx2d, loop_fn, initial_state)
}
