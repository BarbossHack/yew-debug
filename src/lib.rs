mod agent;
mod app;

use agent::Agent;
use wasm_bindgen::prelude::*;
use yew_agent::PublicWorker;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use js_sys::{global, Reflect};

    wasm_logger::init(wasm_logger::Config::default());

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        yew::Renderer::<app::App>::new().render();
    } else {
        Agent::register();
    }
    Ok(())
}
