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
  InitApp,
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
  menu_status: MenuStatus,
  sound: bool
  //
}

impl Agent for Store {
  type Reach = Context<Self>;
  type Message = StoreMsg;
  type Input = StoreInput;
  type Output = StoreOutput;

  fn create(_link: AgentLink<Self>) -> Self {
    //
    // TODO: check si le ls est actif
    //
    let ls_active = false;
    //
    let sound = true; // TODO: connecté le son avec le ls
    //
    Self {
      ls_active,
      sound,
      menu_status: MenuStatus::StartUp
    }
  }

  fn update(&mut self, _msg: Self::Message) {
    //
    // TODO
    //
    //
  }

  fn handle_input(&mut self, input: Self::Input, _id: HandlerId) {
    //
    // TODO
    //
    match input {
      //
      StoreInput:: InitApp => {
        //
        // TODO: renvoyer la balle à SoeMenu
        //
        //
      }
      //
      StoreInput::NoOp => ()
      //
    }
  }
}

