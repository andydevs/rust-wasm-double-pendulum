use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn animation_frame_loop<F: FnMut(f64) + 'static>(mut callback: F) -> Result<(), JsValue> {
    let callback_ptr0 = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
    let callback_ptr1 = callback_ptr0.clone();
    *callback_ptr1.borrow_mut() = Some(Closure::new(move |timestamp| {
        callback(timestamp);
        let _ = window().unwrap().request_animation_frame(
            callback_ptr0
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        );
    }));
    window().unwrap().request_animation_frame(
        callback_ptr1
            .borrow()
            .as_ref()
            .unwrap()
            .as_ref()
            .unchecked_ref(),
    )?;
    Ok(())
}
