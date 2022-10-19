use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew_agent::use_bridge;

mod menu;
mod store;

use menu::MenuStatus;
use store::{Store, StoreInput};

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

#[function_component(SoeMenu)]
fn soe_menu() -> Html {
  let store = { use_bridge::<Store, _>(|_| ()) };
  let curr_state = use_state(|| MenuStatus::StartUp);
  //let init = use_state(|| false);
  //if !*init { init.set(true); }
  //
  // TODO
  log("reached soe menu");
  //
  match *curr_state {
    MenuStatus::Config => {
      //
      // TODO: active / deactive sound
      // TODO: active / deactive ls
      // TODO: select color
      //
      html! {"this is confgi"}
      //
    }
    MenuStatus::Name => {
      //
      // TODO
      //
      html! {"name ?"}
      //
    }
    MenuStatus::Score => {
      //
      // TODO
      //
      html! {"houu! scoring!"}
      //
    }
    MenuStatus::StartUp => {
      //
      // TODO
      //
      //
      //
      html! {
        <div>
          <button>{"Jouer !"}</button>
          <br />
          <button>{"Config"}</button>
          <div>
            <button>{"(sound)"}</button>
          </div>
        </div>
      }
    }
  }
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

