use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  pub order_no: String,
}

#[function_component(OrderCard)]
pub fn order_card(props: &Props) -> Html {
  html! {
    <div class="text-center p-2 border border-dotted rounded-xl bg-red-300 text-white">
      {&props.order_no}
    </div>
  }
}