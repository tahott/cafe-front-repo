use std::{rc::Rc};
use gloo::{timers::callback::Interval, events::EventListener};
use js_sys::{JsString};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventSource, MessageEvent};
use yew::{prelude::*, virtual_dom::VNode};

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
      StateAction::ActionHappend => Self { seconds: 0, is_initial_screen: false }.into(),
    }
  }
}

enum OrderAction {
  MakeOrder(String, VNode),
  CompleteOrder(String),
}

#[derive(Default, Clone)]
struct OrderState {
  waiting_order: Vec<(String, VNode)>,
}

impl Reducible for OrderState {
  type Action = OrderAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      OrderAction::MakeOrder(s, v) => {
        let mut wo = self.waiting_order.clone();
        wo.push((s, v));

        Self { waiting_order: wo }.into()
      },
      OrderAction::CompleteOrder(complete_order) => {
        let wo = self.waiting_order.clone();

        let wo = wo.into_iter().filter(|f| f.0 != complete_order).collect::<Vec<(String, VNode)>>();
        Self { waiting_order: wo.clone() }.into()
      },
    }
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let finished_orders = use_state(Vec::new);
  let seconds_state_handle = use_reducer(|| SecondState { is_initial_screen: true, ..Default::default() });
  let waiting_order_handle = use_reducer(|| OrderState { waiting_order: vec![] });

  let onclick = {
    let seconds_state_handle = seconds_state_handle.clone();
    Callback::from(move |_| {
      seconds_state_handle.dispatch(StateAction::ActionHappend);
    })
  };

  let mut diff_vec: Vec<JsValue> = vec![];

  use_effect_with_deps(
    {
      let fo = finished_orders.clone();
      let mut finished_orders = (*finished_orders).clone();

      let woh = waiting_order_handle.clone();

      move |_| {
        let event_source = EventSource::new("http://localhost:3002").unwrap();
        let listener = EventListener::new(&event_source, "message", move |event: &Event| {
          let event = event.dyn_ref::<MessageEvent>().unwrap();
          let text = event.data().dyn_into::<JsString>().unwrap();
          let text_to_vec = text.split(",").to_vec();

          if text_to_vec.len() > 0 {
            text_to_vec.into_iter().for_each(|o| {
              if diff_vec.contains(&o) == false {
                diff_vec.push(o.clone());
                let order = o.as_string().unwrap();

                finished_orders.push((order.clone(), html! {
                  <OrderCard order_no={order.clone()} />
                }));

                woh.dispatch(OrderAction::CompleteOrder(order));
              }
            });
          }

          fo.set(finished_orders.clone());
        });

        || drop(listener)
      }
    }, 
    (),
  );

  let handle_add_to_waiting_order = {
    let woh = waiting_order_handle.clone();
    Callback::from(move |order_no: String| {
      let order_no = order_no.to_owned();

      let html = html! {
        <OrderCard order_no={order_no.clone()} />
      };

      woh.dispatch(OrderAction::MakeOrder(order_no.clone(), html.clone()));
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
    <div class="container mx-auto h-screen grid grid-cols-1 gap-4 md:grid-cols-2">
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
        <div class="h-2/6">
          <div class="text-lg">{"음료 준비 중 | Preparing"}</div>
          <div class="m-1 grid grid-cols-4 gap-4">
            {waiting_order_handle.waiting_order.iter().map(|tuple| tuple.1.clone()).collect::<Vec<VNode>>().clone()}
          </div>
        </div>
        <div class="h-2/6">
          <div class="text-lg">{"음료 준비 완료 | Complete"}</div>
          <div class="m-1 grid grid-cols-4 gap-4">
            {(*finished_orders).iter().map(|tuple| tuple.1.clone()).collect::<Vec<VNode>>().clone()}
          </div>
        </div>
      </div>

    </div>
  }
}