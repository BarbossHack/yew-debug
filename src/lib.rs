mod agent;
mod app;

use agent::Agent;
use wasm_bindgen::prelude::*;
use yew_agent::PublicWorker;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    Agent::register();
    yew::Renderer::<app::App>::new().render();
    Ok(())
}
