use std::rc::Rc;
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::InitialScreen;
use crate::components::Products;

enum StateAction {
  Default,
  SecondsIncrement,
  ActionHappend,
}

#[derive(Default)]
struct SecondState {
  seconds: usize,
  is_initial_screen: bool,
}

impl Reducible for SecondState {
  type Action = StateAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      StateAction::Default => Self { seconds: 0, is_initial_screen: true }.into(),
      StateAction::SecondsIncrement => Self { seconds: self.seconds + 1, is_initial_screen: self.is_initial_screen }.into(),
      StateAction::ActionHappend => Self { seconds: 0, is_initial_screen: false }.into()
    }
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let seconds_state_handle = use_reducer(|| SecondState { is_initial_screen: true, ..Default::default() });

  let onclick = {
    let seconds_state_handle = seconds_state_handle.clone();
    Callback::from(move |_| {
      seconds_state_handle.dispatch(StateAction::ActionHappend);
    })
  };

  use_effect_with_deps(
    {
      let seconds_state_handle = seconds_state_handle.clone();

      move |_| {
        let interval = Interval::new(1000, move || seconds_state_handle.dispatch(StateAction::SecondsIncrement));
        
        move || drop(interval)
      }
    },
    (),
  );

  if seconds_state_handle.seconds == 30 {
    seconds_state_handle.dispatch(StateAction::Default);
  }

  html! {
    <div class="container mx-auto h-screen">
      {
        match seconds_state_handle.is_initial_screen {
          true => html! {
            <div class="h-screen" {onclick}>
              <InitialScreen />
            </div>
          },
          false => html! {
            <Products />
          }
        }
      }
    </div>
  }
}