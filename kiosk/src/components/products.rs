use std::collections::HashMap;

use itertools::Itertools;
use yew::prelude::*;

use crate::components::MenuCard;

#[derive(PartialEq, Clone, Copy)]
pub enum BeverageType {
  TOTAL,
  COFFEE,
  TEA,
}

#[derive(PartialEq, Clone)]
pub struct Menu {
  pub beverage_type: BeverageType,
  pub name: String,
  pub price: u16,
}

pub enum Msg {
  ChangeTab(BeverageType),
  AddToCart(Menu),
}

pub struct Products {
  total_menu_list: Vec<Menu>,
  current_tab_list: Vec<Html>,
  cart: Vec<Menu>,
  cart_price: u16,
}

impl Component for Products {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let add_to_cart = ctx.link().callback(|menu| Msg::AddToCart(menu));

    let total_menu_list: Vec<Menu> = vec![
      Menu { beverage_type: BeverageType::COFFEE, name: "에스프레소".to_string(), price: 3_000 },
      Menu { beverage_type: BeverageType::COFFEE, name: "아메리카노".to_string(), price: 3_500 },
      Menu { beverage_type: BeverageType::TEA, name: "홍차".to_string(), price: 4_000 },
    ];

    let current_tab_list: Vec<Html> = total_menu_list.iter().map(|data| {
      html! {
        <MenuCard menu={data.clone()} add_to_cart={add_to_cart.clone()} />
      }
    }).collect();

    Self {
      total_menu_list,
      current_tab_list,
      cart: vec![],
      cart_price: 0,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    let add_to_cart = ctx.link().callback(|menu| Msg::AddToCart(menu));

    match msg {
      Msg::ChangeTab(key) => {
        match key {
          BeverageType::TOTAL => {
            self.current_tab_list = self.total_menu_list.iter().map(|data| {
              html! {
                <MenuCard menu={data.clone()} add_to_cart={add_to_cart.clone()} />
              }
            }).collect();
          },
          BeverageType::COFFEE => {
            self.current_tab_list = self.total_menu_list.iter().filter(|data| {
              data.beverage_type == BeverageType::COFFEE
            }).map(|data| {
              html! {
                <MenuCard menu={data.clone()} add_to_cart={add_to_cart.clone()} />
              }
            }).collect();
          },
          BeverageType::TEA => {
            self.current_tab_list = self.total_menu_list.iter().filter(|data| {
              data.beverage_type == BeverageType::TEA
            }).map(|data| {
              html! {
                <MenuCard menu={data.clone()} add_to_cart={add_to_cart.clone()} />
              }
            }).collect();
          },
        }
        true
      },
      Msg::AddToCart(menu) => {
        self.cart_price += menu.price;
        self.cart.push(menu);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let beverage_tabs: Vec<Html> = self.total_menu_list.iter().fold(vec![BeverageType::TOTAL], |mut init, data| {
      let find = init.iter().find(|x| **x == data.beverage_type);
      match find {
        Some(_) => init,
        None => {
          init.push(data.beverage_type);
          init
        },
      }
    }).iter().map(|data| {
      let key = data.clone();
      html! {
        <li class="mr-2" onclick={ ctx.link().callback(move |_| Msg::ChangeTab(key)) }>
          <a href="#" class="inline-block p-4 rounded-t-lg border-b-2 border-transparent hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300">
            {
              match data {
                BeverageType::TOTAL => "전체".to_string(),
                BeverageType::COFFEE => "커피".to_string(),
                BeverageType::TEA => "차".to_string(),
              }
            }
          </a>
        </li>
      }
    }).collect();

    let cart_state = self.cart.is_empty();

    html! {
      <div>
        <div class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
          <ul class="flex flex-wrap -mb-px">
            {beverage_tabs}
          </ul>
        </div>
        <div class="m-2 p-0 pl-[16px] pr-[16px] grid grid-cols-4 gap-[8px] md:grid-cols-8 md:gap-[16px] lg:grid-cols-8 lg:gap-[16px] lg:m-6 lg:pl-48 lg:pr-48">
          {self.current_tab_list.clone()}
        </div>
        // cart area
        {
          match !cart_state {
            true => {
              let cart_list: Vec<Html> = self.cart.iter().fold(HashMap::new(), |mut init, data| {
                if init.is_empty() {
                  init.insert(data.name.clone(), 1);
                } else {
                  match init.get(&data.name) {
                    Some(&num) => init.insert(data.name.clone(), num + 1),
                    _ => init.insert(data.name.clone(), 1),
                  };
                }

                init
              }).iter().sorted().map(|(key, value)| {
                html! {
                  <div>{key}{"::"}{value}</div>
                }
              }).collect();
              
              html! {
                <div class="absolute bottom-0 bg-amber-200 w-screen divide-y divide-rose-900 rounded-t-lg p-2">
                  <div>{cart_list}</div>
                  <div>{"total price: "}{self.cart_price}</div>
                </div>
              }
            },
            false => html! {
              <div></div>
            },
          }
        }
      </div>
    }
  }
}