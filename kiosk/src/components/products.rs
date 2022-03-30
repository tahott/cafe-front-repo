use yew::prelude::*;
pub enum BeverageType {
  COFFEE,
  TEA,
}

pub struct Menu {
  beverage_type: BeverageType,
  name: String,
}

pub struct Products {
  list: Vec<Menu>,
}

impl Component for Products {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let list: Vec<Menu> = vec![
      Menu { beverage_type: BeverageType::COFFEE, name: "에스프레소".to_string() },
      Menu { beverage_type: BeverageType::COFFEE, name: "아메리카노".to_string() },
    ];
    Self {
      list,
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let menu: Vec<Html> = self.list.iter().map(|l| {
      html! {
        <div>{l.name.clone()}</div>
      }
    }).collect();
    html! {
      <div>
        {menu}
      </div>
    }
  }
}