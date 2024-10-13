use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn setup(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
  canvas.style().set_property("border", "solid")?;
  canvas.style().set_property("touch-action", "none")?;
  let context = canvas
    .get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  {
    let closure =
      Closure::<dyn FnMut(_)>::new(move |event: web_sys::PointerEvent| {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        if event.pressure() > 0.0 {
          context.line_to(x, y);
          context.stroke();
        } else {
          context.move_to(x, y);
        }
      });
    canvas.add_event_listener_with_callback(
      "pointerrawupdate",
      closure.as_ref().unchecked_ref(),
    )?;
    closure.forget();
  }
  Ok(())
}
