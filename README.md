# Rust Canvas

This is a proof of concept implementation for low latency hand writing in the
browser.

For maximum performance, the time critical event handling is written in Rust and compiled down to WebAssembly using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).

## Background

Popular open source applications like
[Excalidraw](https://github.com/excalidraw/excalidraw) support digital pen input
based on the
[PointerEvent API](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent).
However, while the existing implementations are good for purposes like drawing,
fast hand writing becomes almost impossible due to high latency and dropped events.

As it turns out, this is not a general limitation of the PointerEvent API, at
least if the
[pointerrawupdate event](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerrawupdate_event)
is used.
