use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn run_request_animation_frame_loop<F: FnMut() + 'static>(
    mut callback: F,
) -> Result<(), JsValue> {
    type CallbackPtr = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

    fn animation_frame_loop(frame_ptr: &CallbackPtr) -> Result<(), JsValue> {
        window().unwrap().request_animation_frame(
            frame_ptr
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )?;
        Ok(())
    }

    let callback_ptr0 = Rc::new(RefCell::new(None));
    let callback_ptr1 = callback_ptr0.clone();

    *callback_ptr1.borrow_mut() = Some(Closure::new(move || {
        callback();
        let _ = animation_frame_loop(&callback_ptr0);
    }));
    animation_frame_loop(&callback_ptr1)
}
