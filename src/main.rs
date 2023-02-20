// native app entry_point

use async_std::task::block_on;

use sketch::{Model, run_app};

mod sketch;

fn main() {
    let model = Model {};
    block_on(async {
        run_app(model).await;
    });
}
