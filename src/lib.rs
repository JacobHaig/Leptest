pub mod app;
mod ui;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;
      use ui::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
