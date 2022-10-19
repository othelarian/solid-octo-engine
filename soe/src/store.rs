use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::menu::MenuStatus;

pub enum StoreMsg {
  //
  // TODO
  //
  NoOp
  //
}

pub enum StoreInput {
  //
  // TODO
  //
  NoOp
  //
}

pub enum StoreOutput {
  //
  // TODO
  //
  NoOp
  //
}

pub struct Store {
  //
  ls_active: bool,
  menu_status: MenuStatus
  //
}

impl Agent for Store {
  type Reach = Context<Self>;
  type Message = StoreMsg;
  type Input = StoreInput;
  type Output = StoreOutput;

  fn create(link: AgentLink<Self>) -> Self {
    //
    // TODO: check si le ls est actif
    //
    let ls_active = false;
    //
    //
    Self {
      ls_active,
      menu_status: MenuStatus::StartUp
    }
  }

  fn update(&mut self, msg: Self::Message) {
    //
    // TODO
    //
    //
  }

  fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
    //
    // TODO
    //
  }
}

