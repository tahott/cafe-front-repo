use yew::prelude::*;

#[function_component(OrderCard)]
pub fn order_card() -> Html {
  html! {
    <div class="text-center p-2 border border-dotted rounded-xl bg-red-300 text-white">
      {"001"}
    </div>
  }
}