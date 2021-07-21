#![feature(once_cell)] // 1.53.0-nightly (2021-04-01 d474075a8f28ae9a410e)
use std::{lazy::SyncLazy, sync::Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use piet::*;
use kurbo::*;
use piet_web::*;

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
pub fn make_nes(canvas_id: &str) -> Result<(), JsValue> {
  CPU.lock().unwrap().reset();

  // get canvas and webgl context
	let window = window().unwrap();
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(canvas_id).unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  // let context = canvas
  //   .get_context("webgl2")?
  //   .unwrap()
  //   .dyn_into::<WebGl2RenderingContext>()?;

  // let vert_shader = compile_shader(
  //   &context,
  //   WebGl2RenderingContext::VERTEX_SHADER,
  //   r##"#version 300 es

	// 		in vec4 position;

	// 		void main() {

	// 				gl_Position = position;
	// 		}
	// 		"##,
  // )?;

  // let frag_shader = compile_shader(
  //   &context,
  //   WebGl2RenderingContext::FRAGMENT_SHADER,
  //   r##"#version 300 es

	// 		precision highp float;
	// 		out vec4 outColor;

	// 		void main() {
	// 				outColor = vec4(1, 1, 1, 1);
	// 		}
	// 		"##,
  // )?;

  // let program = link_program(&context, &vert_shader, &frag_shader)?;
  // context.use_program(Some(&program));

  // let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

  // let position_attribute_location = context.get_attrib_location(&program, "position");
  // let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
  // context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

  // // Note that `Float32Array::view` is somewhat dangerous (hence the
  // // `unsafe`!). This is creating a raw view into our module's
  // // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
  // // (aka do a memory allocation in Rust) it'll cause the buffer to change,
  // // causing the `Float32Array` to be invalid.
  // //
  // // As a result, after `Float32Array::view` we have to be very careful not to
  // // do any memory allocations before it's dropped.
  // unsafe {
  //   let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

  //   context.buffer_data_with_array_buffer_view(
  //     WebGl2RenderingContext::ARRAY_BUFFER,
  //     &positions_array_buf_view,
  //     WebGl2RenderingContext::STATIC_DRAW,
  //   );
  // }

  // let vao = context
  //   .create_vertex_array()
  //   .ok_or("Could not create vertex array object")?;
  // context.bind_vertex_array(Some(&vao));

  // context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
  // context.enable_vertex_attrib_array(position_attribute_location as u32);

  // context.bind_vertex_array(Some(&vao));

  // let vert_count = (vertices.len() / 3) as i32;
  // draw(&context, vert_count);

  unsafe {
    console_log!("Hello {}!", canvas_id);
  }
  Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
  context: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}

pub mod color;
pub mod nes;
