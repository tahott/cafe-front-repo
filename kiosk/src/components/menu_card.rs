use yew::prelude::*;

use super::products::Menu;

#[derive(Properties, PartialEq)]
pub struct MenuCardProps {
  pub menu: Menu,
  pub add_to_cart: Callback<Menu>,
}

#[function_component(MenuCard)]
pub fn menu_card(props: &MenuCardProps) -> Html {
  let onclick = {
    let add_to_cart = props.add_to_cart.clone();
    let menu = props.menu.clone();
    Callback::from(move |_| {
      add_to_cart.emit(menu.clone());
    })
  };

  html! {
    <div class="menu_card_container">
      <div class="menu_card grid justify-items-center">
        <div class="w-[64px] h-[64px] grid justify-items-center items-center rounded-lg border opacity-50 lg:w-[96px] lg:h-[96px]">
          <img src="" />
        </div>
        <p class="truncate text-center text-sm">{&props.menu.name}</p>
        <p class="text-center text-xs">{"₩"}{&props.menu.price}</p>
        <button {onclick}>{"담기"}</button>
      </div>
    </div>
  }
}