use std::rc::Rc;
use gloo::{timers::callback::Interval, events::EventListener, console::console};
use js_sys::{ArrayBuffer, JsString};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventSource, MessageEvent, Blob, console};
use yew::prelude::*;

use crate::components::{InitialScreen, Products, OrderCard};

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
  let waiting_orders = use_state(Vec::new);
  let seconds_state_handle = use_reducer(|| SecondState { is_initial_screen: true, ..Default::default() });

  let onclick = {
    let seconds_state_handle = seconds_state_handle.clone();
    Callback::from(move |_| {
      seconds_state_handle.dispatch(StateAction::ActionHappend);
    })
  };

  use_effect_with_deps(move |_| {
    let event_source = EventSource::new("http://localhost:3002").unwrap();
    let listener = EventListener::new(&event_source, "message", move |event: &Event| {
      let event = event.dyn_ref::<MessageEvent>().unwrap();

      let text = event.data().dyn_into::<JsString>().unwrap();
      let text_to_vec = text.split(",").to_vec();

      if text_to_vec.len() > 0 {
        let res = text_to_vec.iter().map(|s| {
          s.as_string().unwrap()
        }).collect::<Vec<String>>();
      }
    });

    || drop(listener)
  }, ());

  let handle_add_to_waiting_order = {
    let waiting_orders = waiting_orders.clone();
    Callback::from(move |order_no: String| {
      let order_no = order_no.to_owned();
      let mut new_order = (*waiting_orders).clone();

      new_order.push(html! {
        <OrderCard order_no={order_no} />
      });
      waiting_orders.set(new_order);
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

  if seconds_state_handle.seconds == 60 {
    seconds_state_handle.dispatch(StateAction::Default);
  }

  html! {
    <div class="container mx-auto h-screen grid grid-cols-1 md:grid-cols-2">
      <div>
        {
          match seconds_state_handle.is_initial_screen {
            true => html! {
              <div class="h-screen" {onclick}>
                <InitialScreen />
              </div>
            },
            false => html! {
              <div class="h-screen"><Products add_to_wating_order={handle_add_to_waiting_order.clone()} /></div>
            }
          }
        }
      </div>
      <div class="mx-auto w-full h-screen">
        <div class="m-1 border border-dotted grid grid-cols-4 gap-4">
          {(*waiting_orders).clone()}
        </div>
        <div class="m-1 border border-dotted grid grid-cols-4 gap-4">{"pick up noti"}</div>
      </div>

    </div>
  }
}