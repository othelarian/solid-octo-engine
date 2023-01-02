use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::store::{Store, StoreInput};

pub enum MenuStatus {
  Config,
  Loading,
  Name,
  Score,
  StartUp
}

pub enum SoeMenuMsg {
  NoOp
}

pub struct SoeMenu {
  curr_state: MenuStatus,
  store: Box<dyn Bridge<Store>>
}

impl Component for SoeMenu {
  type Message = SoeMenuMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let mut store = Store::bridge(ctx.link().callback(|res| match res {
      //
      // TODO
      //
      _ => SoeMenuMsg::NoOp
    }));
    store.send(StoreInput::InitApp);
    Self { curr_state: MenuStatus::Loading, store}
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      //
      // TODO
      //
      SoeMenuMsg::NoOp => false
      //
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    //
    // TODO
    //
    //
    //
    match self.curr_state {
      MenuStatus::Config => {
        //
        // TODO: active / deactive sound
        // TODO: active / deactive ls
        // TODO: select color
        //
        html! {"this is confgi"}
        //
      }
      MenuStatus::Loading => {
        //
        // TODO
        //
        html! {"Chargement ..."}
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
          <>
            <button>{"\u{f04b}"}</button>
            <br />
            <button>{"\u{f085}"}</button>
            <div id="soe-menu-sound">
              <button>{"\u{f886}\u{f889}"}</button>
            </div>
          </>
        }
      }
    }
  }
}

#[function_component(Config)]
fn config() -> Html {
  //
  // TODO
  //
  html! {"config (not ready yet)"}
  //
}

