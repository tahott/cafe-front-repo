use yew::prelude::*;

pub struct InitialScreen {}

impl Component for InitialScreen {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="text-center text-2xl lg:text-4xl">{"주문 하시려면 화면을 터치해 주세요"}</div>
    }
  }
}

