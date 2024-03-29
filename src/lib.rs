mod app;
mod pages;
mod components;
mod store;
mod types;

use yewdux::prelude::WithDispatch;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<WithDispatch<app::App>>();

    Ok(())
}
