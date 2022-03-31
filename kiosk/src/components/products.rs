use yew::prelude::*;

use crate::components::MenuCard;

#[derive(PartialEq, Clone, Copy)]
pub enum BeverageType {
  TOTAL,
  COFFEE,
  TEA,
}

pub struct Menu {
  beverage_type: BeverageType,
  name: String,
  price: u16,
}

pub enum Msg {
  ChangeTab(BeverageType),
}

pub struct Products {
  total_menu_list: Vec<Menu>,
  current_tab_list: Vec<Html>,
}

impl Component for Products {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let total_menu_list: Vec<Menu> = vec![
      Menu { beverage_type: BeverageType::COFFEE, name: "에스프레소".to_string(), price: 3_000 },
      Menu { beverage_type: BeverageType::COFFEE, name: "아메리카노".to_string(), price: 3_500 },
      Menu { beverage_type: BeverageType::TEA, name: "홍차".to_string(), price: 4_000 },
    ];

    let current_tab_list: Vec<Html> = total_menu_list.iter().map(|data| {
      html! {
        <MenuCard name={data.name.clone()} price={data.price} />
      }
    }).collect();

    Self {
      total_menu_list,
      current_tab_list,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::ChangeTab(key) => {
        match key {
          BeverageType::TOTAL => {
            self.current_tab_list = self.total_menu_list.iter().map(|data| {
              html! {
                <MenuCard name={data.name.clone()} price={data.price} />
              }
            }).collect();
          },
          BeverageType::COFFEE => {
            self.current_tab_list = self.total_menu_list.iter().filter(|data| {
              data.beverage_type == BeverageType::COFFEE
            }).map(|data| {
              html! {
                <MenuCard name={data.name.clone()} price={data.price} />
              }
            }).collect();
          },
          BeverageType::TEA => {
            self.current_tab_list = self.total_menu_list.iter().filter(|data| {
              data.beverage_type == BeverageType::TEA
            }).map(|data| {
              html! {
                <MenuCard name={data.name.clone()} price={data.price} />
              }
            }).collect();
          },
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
      </div>
    }
  }
}