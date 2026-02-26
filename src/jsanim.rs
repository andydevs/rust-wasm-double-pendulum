use std::f64::consts::TAU;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[allow(dead_code)]
pub struct FrameCtx {
    pub frame: u32,
    pub dt: f64,
    pub timestamp: f64,
}

pub struct WindowCtx {
    pub ctx: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

impl WindowCtx {
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx, canvas }
    }

    pub fn clear(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, width, height);
    }

    pub fn circle(&self, x: f64, y: f64, r: f64, color: &str) {
        self.ctx.set_fill_style_str(color);
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, TAU).unwrap();
        self.ctx.fill();
    }
}

pub fn animation_frame_loop<F: FnMut(f64) + 'static>(mut callback: F) -> Result<(), JsValue> {
    type CallbackPtr = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
    let callback_ptr0 = Rc::new(RefCell::new(None));
    let callback_ptr1 = Rc::clone(&callback_ptr0);
    fn request_frame(callback: &CallbackPtr) -> Result<(), JsValue> {
        window().unwrap().request_animation_frame(
            callback.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )?;
        Ok(())
    }
    *callback_ptr1.borrow_mut() = Some(Closure::new(move |timestamp| {
        callback(timestamp);
        request_frame(&callback_ptr0).unwrap();
    }));
    request_frame(&callback_ptr1)
}
