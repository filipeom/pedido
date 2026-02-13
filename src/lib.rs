use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub fn escape(screen_width: f64, screen_height: f64) -> Coords {
    let button_width = 100.0;
    let button_height = 50.0;

    let new_x = js_sys::Math::random() * (screen_width - button_width);
    let new_y = js_sys::Math::random() * (screen_height - button_height);
    Coords {
        x: new_x,
        y: new_y
    }
}
