use crate::{ErrorMessage, PaddleResult};
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[must_use = "If this is dropped, the thread will be stopped"]
pub struct ThreadHandler {
    handle: i32,
    function: FunctionType,
}

type SimpleThreadClosure = Closure<dyn FnMut()>;
type AnimationFrameClosure = Closure<dyn FnMut(f64)>;

enum FunctionType {
    AnimationFrame(Rc<RefCell<Option<AnimationFrameClosure>>>),
    Interval(SimpleThreadClosure),
    Timeout(SimpleThreadClosure),
}

impl Drop for ThreadHandler {
    fn drop(&mut self) {
        match &mut self.function {
            FunctionType::AnimationFrame(f) => {
                let _ = web_sys::window()
                    .unwrap()
                    .cancel_animation_frame(self.handle);
                *f.borrow_mut() = None;
            }
            FunctionType::Interval(_) => {
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(self.handle);
            }
            FunctionType::Timeout(_) => {
                web_sys::window()
                    .unwrap()
                    .clear_timeout_with_handle(self.handle);
            }
        }
    }
}

/// Sets up a request animation frame loop
pub fn start_drawing_thread(mut f: impl FnMut(f64) + 'static) -> PaddleResult<ThreadHandler> {
    // Allocate some memory for a function pointer and initialize it with a null pointer
    let function = Rc::new(RefCell::new(None));
    let function_alias = function.clone();

    let closure = move |dt: f64| {
        if let Some(function) = function.borrow().as_ref() {
            f(dt);
            request_animation_frame(function).expect("RAF failed");
        }
        // else: Handle has been dropped, this means no more drawing
    };
    *function_alias.borrow_mut() = Some(Closure::<dyn FnMut(f64)>::wrap(
        Box::new(closure) as Box<dyn FnMut(f64)>
    ));

    let handle = request_animation_frame(function_alias.borrow().as_ref().unwrap())?;
    Ok(ThreadHandler {
        function: FunctionType::AnimationFrame(function_alias),
        handle,
    })
}

// Only requests a single frame, needs to be called repeatedly for each frame.
fn request_animation_frame(function: &Closure<dyn FnMut(f64)>) -> PaddleResult<i32> {
    let handle = web_sys::window()
        .unwrap()
        .request_animation_frame(function.as_ref().dyn_ref().unwrap())
        .map_err(|_| ErrorMessage::technical("Failed on request_animation_frame".to_owned()))?;
    Ok(handle)
}

pub fn start_thread(f: impl FnMut() + 'static, timeout_ms: i32) -> PaddleResult<ThreadHandler> {
    let function = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            function.as_ref().dyn_ref().unwrap(),
            timeout_ms,
        )
        .map(|handle| ThreadHandler {
            function: FunctionType::Interval(function),
            handle,
        })
        .map_err(|_| ErrorMessage::technical("Failed on set_interval_with_callback".to_owned()))
}

pub fn create_thread(f: impl FnMut() + 'static) -> ThreadHandler {
    let function = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
    ThreadHandler {
        function: FunctionType::Timeout(function),
        handle: 0,
    }
}

impl ThreadHandler {
    pub fn set_timeout(&mut self, timeout_ms: i32) -> PaddleResult<()> {
        match &mut self.function {
            FunctionType::Timeout(function) | FunctionType::Interval(function) => {
                let handle = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        function.as_ref().dyn_ref().unwrap(),
                        timeout_ms,
                    )
                    .map_err(|_| {
                        ErrorMessage::technical("Failed on set_timeout_with_callback".to_owned())
                    })?;
                self.handle = handle;
            }
            FunctionType::AnimationFrame(_) => panic!("Called set_timeout on drawing thread."),
        }
        Ok(())
    }
}

pub(crate) fn register_debug_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(|panic_info| {
        let nuts_info = nuts::panic_info();
        web_sys::console::error_1(&nuts_info.into());
        console_error_panic_hook::hook(panic_info);
    }));
}
