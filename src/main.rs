// native app entry_point

mod sketch;
use sketch::{Model,run_app};
use async_std::task::block_on;
fn main(){
    let model = Model{};
    block_on(async {
        run_app(model).await;
    });
}