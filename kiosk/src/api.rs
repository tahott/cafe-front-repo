use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};

use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response, console};

use crate::components::Menu;

#[derive(Serialize, Deserialize)]
struct OrderItem {
  name: String,
  size: String,
  amount: u8,
  price: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize="camelCase"))]
struct Payment {
  vendor: String,
  approval_value: String,
}

#[derive(Serialize, Deserialize)]
struct Order {
  order: Vec<OrderItem>,
  payment: Payment,
  takeout: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize="camelCase"))]
pub struct Receipt {
  order: Vec<OrderItem>,
  pub order_no: String,
  order_created_at: String,
  takeout: bool,
  pub payment_result: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
  err: JsValue,
}

impl Display for FetchError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(&self.err, f)
  }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
  fn from(value: JsValue) -> Self {
    Self { err: value }
  }
}

pub async fn send_order() -> Result<Receipt, FetchError> {
  let mut opts = RequestInit::new();
  opts.method("POST");
  opts.mode(RequestMode::Cors);
  let body = JsValue::from_str(&serde_json::to_string(&Order {
    order: vec![OrderItem {
      name: "아메리카노".to_owned(),
      size: "small".to_owned(),
      amount: 1,
      price: 3_500,
    }],
    payment: Payment { vendor: "naver".to_owned(), approval_value: "abc1q2w".to_owned() },
    takeout: false
  }).unwrap());

  opts.body(Some(&body));

  let request = Request::new_with_str_and_init(
    "http://localhost:3000/order",
    &opts,
  )?;

  request.headers().set("Content-Type", "application/json")?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp = resp_value.dyn_into::<Response>().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  console::log_1(&json);

  let receipt = json.into_serde().unwrap();
  
  Ok(receipt)
}

pub async fn get_menu() -> Result<Vec<Menu>, FetchError> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(
    "http://localhost:3000/order",
    &opts,
  )?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp = resp_value.dyn_into::<Response>().unwrap();

  let json = JsFuture::from(resp.json()?).await?;

  let menu: Vec<Menu> = json.into_serde().unwrap();

  Ok(menu)
}
