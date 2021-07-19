#![feature(once_cell)] // 1.53.0-nightly (2021-04-01 d474075a8f28ae9a410e)
use std::{lazy::SyncLazy, sync::Mutex};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CPU: SyncLazy<Mutex<nes::cpu::CPU>> = SyncLazy::new(|| Mutex::new(nes::cpu::CPU::new()));

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

macro_rules! console_log {
	// Note that this is using the `log` function imported above during
	// `bare_bones`
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn make_nes(canvas_id: &str) {
  unsafe {
    console_log!("Hello {}!", canvas_id);
  }
  CPU.lock().unwrap().reset();
}

pub mod nes;
