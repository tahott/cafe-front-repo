use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuCardProps {
  pub name: String,
  pub price: u16,
}

#[function_component(MenuCard)]
pub fn menu_card(props: &MenuCardProps) -> Html {
  html! {
    <div class="menu_card_container">
      <div class="menu_card grid justify-items-center">
        <div class="w-[64px] h-[64px] grid justify-items-center items-center rounded-lg border opacity-50 lg:w-[96px] lg:h-[96px]">
          <img src="" />
        </div>
        <p class="truncate text-center text-sm">{&props.name}</p>
        <p class="text-center text-xs">{"₩"}{&props.price}</p>
      </div>
    </div>
  }
}