use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(not(target_arch = "wasm32"))]
pub use async_std::task::spawn;
pub use async_std::task::block_on;
use crate::sketch::{Model,run_app};
#[wasm_bindgen]
pub async fn main_web(){
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = Model{};
    block_on(async {
        run_app(model).await;
    });

}