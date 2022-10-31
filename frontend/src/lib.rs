mod app;
mod components;
mod css;
mod page;

use seed::prelude::wasm_bindgen;
use seed::App;

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", app::init, app::update, app::view);
}

