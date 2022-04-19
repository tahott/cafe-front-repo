use std::{collections::HashMap, rc::Rc};
use itertools::Itertools;
use yew::prelude::*;

use crate::{components::MenuCard, api::{send_order, FetchError, Receipt}};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BeverageType {
  TOTAL,
  COFFEE,
  TEA,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Menu {
  pub beverage_type: BeverageType,
  pub name: String,
  pub price: u16,
}

pub enum Operator {
  ADD,
  SUB,
}

pub enum FetchState<T> {
  Success(T),
  Failed(FetchError),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  pub add_to_wating_order: Callback<String>,
}

pub enum Msg {
  ChangeTab(BeverageType),
  AddToCart(Menu),
  ChageCartAmount(Operator, Rc<Menu>),
  SendOrder,
  SetOrderFetchState(FetchState<Receipt>)
}

pub struct Products {
  total_menu_list: Vec<Menu>,
  current_tab_list: Vec<Html>,
  cart: Vec<Menu>,
  cart_price: u16,
}

impl Component for Products {
  type Message = Msg;
  type Properties = Props;

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
      },
      Msg::ChageCartAmount(opt, menu) => {
        match opt {
          Operator::ADD => {
            self.cart.push(Menu { beverage_type: menu.beverage_type, name: menu.name.clone(), price: menu.price  });
            self.cart_price += menu.price;
          },
          Operator::SUB => {
            if let Some(idx) = self.cart.iter().position(|data| data.name == menu.name) {
              self.cart.swap_remove(idx);
              self.cart_price -= menu.price;
            }
          },
        }
        true
      },
      Msg::SendOrder => {
        ctx.link().send_future(async {
          match send_order().await {
            Ok(receipt) => Msg::SetOrderFetchState(FetchState::Success(receipt)),
            Err(err) => Msg::SetOrderFetchState(FetchState::Failed(err)),
          }
        });
        true
      },
      Msg::SetOrderFetchState(fetch_state) => {
        match fetch_state {
            FetchState::Success(receipt) => {
              if receipt.payment_result == true {
                ctx.props().add_to_wating_order.emit(receipt.order_no.to_owned());
              }
            },
            FetchState::Failed(e) => todo!(),
        }
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
        <div class="m-2 p-0 pl-[16px] pr-[16px] grid grid-cols-4 gap-[8px] md:grid-cols-8 md:gap-[16px] lg:grid-cols-8 lg:gap-[16px]">
          {self.current_tab_list.clone()}
        </div>
        // cart area
        {
          match !cart_state {
            true => {
              let cart = self.cart.clone();
              let cart_list: Vec<Html> = cart.into_iter().fold(HashMap::new(), |mut init, data| {
                if init.is_empty() {
                  init.insert(data, 1);
                } else {
                  match init.get(&data) {
                    Some(&num) => init.insert(data, num + 1),
                    _ => init.insert(data, 1),
                  };
                }

                init
              }).iter().sorted().map(|(menu, amount)| {
                let menu = Rc::new(menu.clone());
                let cart_menu_sub = Rc::clone(&menu);
                let cart_menu_add = Rc::clone(&menu);
                html! {
                  <div class="flex justify-between my-1">
                    <div>{menu.name.clone()}</div>
                    <div class="inline-flex">
                      <button type="button" onclick={ctx.link().callback(move |_| Msg::ChageCartAmount(Operator::SUB, cart_menu_add.to_owned()))} class="rounded-l inline-block px-3 py-1.25 bg-yellow-500 text-white font-medium text-xs leading-tight">{"-"}</button>
                      <button disabled={true} class="inline-block px-3 py-1.25 bg-yellow-500 text-white font-medium text-xs leading-tight">{amount}</button>
                      <button type="button" onclick={ctx.link().callback(move |_| Msg::ChageCartAmount(Operator::ADD, cart_menu_sub.to_owned()))} class="rounded-r inline-block px-3 py-1.25 bg-yellow-500 text-white font-medium text-xs leading-tight">{"+"}</button>
                    </div>
                  </div>
                }
              }).collect();
              
              html! {
                <div class="container absolute bottom-0 bg-amber-200 w-screen divide-y divide-rose-900 rounded-t-lg p-2">
                  <div>{cart_list}</div>
                  <div>{"₩ "}{self.cart_price}</div>
                  <div onclick={ctx.link().callback(|_| Msg::SendOrder)}>{"주문하기"}</div>
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