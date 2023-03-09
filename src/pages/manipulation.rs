use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use yew_style_in_rs::*;
use crate::css;

pub struct Manipulation{
    test_string: String,
    stl: HashMap<String,String>
}

pub enum Msg {
  TestString(String)
}

impl Component for Manipulation {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {test_string: "somethingsomething".to_string(), stl: css::manipulation::styles()}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <div style={self.stl["main"].clone()}>
            <div style={self.stl["newspapertitle"].clone()} class="titleheader">
                {"Manipulation"}
            </div>
            <div>
                <a href="/manipulation">{"A Course in the Dark Arts"}</a>
            </div>
        </div>
    }
  }
}

