#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
  unsafe {
    alert(&format!("Hello, {}!", name));
  }
}

pub mod nes;
