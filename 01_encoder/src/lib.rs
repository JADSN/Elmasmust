use wasm_bindgen::prelude::*;

mod action;
mod actions;
mod cmd;
mod dispatcher;

#[macro_use]
mod macros;

use cmd::Cmd;
use dispatcher::Dispatcher;

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    }
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub fn entrypoint(input: &str) -> String {
    let data = input.replace(' ', "");
    let data = data.trim();
    log!("[Input]: {}", data);

    match serde_json::from_str::<Cmd>(&data) {
        Ok(cmd) => Dispatcher::from_cmd(&cmd),
        Err(error) => {
            log!("[ERROR] {:?}", error);
            String::from("Oops")
        }
    }
}
