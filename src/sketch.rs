// P_1_0_01
//
// Generative Gestaltung – Creative Coding im Web
// ISBN: 978-3-87439-902-9, First Edition, Hermann Schmidt, Mainz, 2018
// Benedikt Groß, Hartmut Bohnacker, Julia Laub, Claudius Lazzeroni
// with contributions by Joey Lee and Niels Poldervaart
// Copyright 2018
//
// http://www.generative-gestaltung.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
/*
* changing colors and size by moving the mouse
*
* MOUSE
* position x          : size
* position y          : color
*
* KEYS
* s                   : save png
*/

use wasm_bindgen::prelude::wasm_bindgen;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
pub use async_std::task::spawn;

#[cfg(target_arch = "wasm32")]
pub use async_std::task::spawn_local as spawn;

pub use async_std::task::block_on;


pub struct Model;


fn update(app: &App, model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    draw_circle(&draw, 0.0, 0.0, 200.0);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

// Recursive function
fn draw_circle(draw: &Draw, x: f32, y: f32, r: f32) {
    let norm_radius = map_range(r, 2.0, 360.0, 0.0, 1.0);
    draw.ellipse()
        .x_y(x, y)
        .radius(r)
        .hsva(norm_radius, 0.75, 1.0, norm_radius)
        .stroke(BLACK);

    if r > 8.0 {
        // Four circles! left right, up and down
        draw_circle(&draw, x + r, y, r / 2.0);
        draw_circle(&draw, x - r, y, r / 2.0);
        draw_circle(&draw, x, y + r, r / 2.0);
        draw_circle(&draw, x, y - r, r / 2.0);
    }
}


#[wasm_bindgen]
pub async fn run() ->bool{
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = Model{};
    block_on(async {
        run_app(model).await;
    });
    
    true
}


pub async fn run_app(model: Model) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    MODEL.with(|m| m.borrow_mut().replace(model));

    app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    .update(update)
    .run_async()
    .await;
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .title("web test")
        // .raw_event(model::raw_event)
        // .key_pressed(key_pressed)
        // .key_released(key_released)
        // .mouse_pressed(mouse_pressed)
        // .mouse_moved(model::mouse_moved)
        // .mouse_released(model::mouse_released)
        // .mouse_wheel(model::mouse_wheel)
        // .touch(model::touch)

        .view(view)
        .build_async()
        .await
        .unwrap();
}


