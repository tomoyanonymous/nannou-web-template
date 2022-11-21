use wasm_bindgen::prelude::wasm_bindgen;

mod sketch;
use sketch::{Model,run_app};
use async_std::task::block_on;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web(){
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = Model{};
    block_on(async {
        run_app(model).await;
    });

}