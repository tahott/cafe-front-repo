use yew::prelude::*;
use gloo::dialogs::alert;

pub struct App {}

pub enum Msg {
  ScreenTouch,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::ScreenTouch => {
        alert("화면을 터치하였습니다");
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="container mx-auto p-4 h-screen" onclick={ ctx.link().callback(|_| Msg::ScreenTouch) }>
        <div class="text-center text-4xl">{"주문 하시려면 화면을 터치해 주세요"}</div>
      </div>
    }
  }
}