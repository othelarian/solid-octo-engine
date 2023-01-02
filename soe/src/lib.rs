use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

mod menu;
mod store;

use menu::SoeMenu;

#[function_component(SoeGame)]
fn soe_game() -> Html {
  //
  //
  // TODO
  //
  log("reach soe_game");
  //
  html! {"oupsie"}
  //
}


struct App { pub menu: Html }

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    clearMenu();
    let menu = create_portal(
      html!{<SoeMenu />},
      gloo_utils::document().get_element_by_id("soe-menu").unwrap()
    );
    Self { menu }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {<>{self.menu.clone()}<SoeGame /></>}
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = SOE)]
  fn clearMenu();

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn start_soe(game: Element) { yew::start_app_in_element::<App>(game); }

