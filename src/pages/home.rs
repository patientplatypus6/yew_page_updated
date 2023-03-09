use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use yew_style_in_rs::*;
use crate::css;

pub struct Home{
    test_string: String,
    stl: HashMap<String,String>
}

pub enum Msg {
    TestString(String)
}

async fn get_test() -> Result<(), reqwest::Error> {

    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/test")
        .header(USER_AGENT, "GET_TEST")
        .send()
        .await?;
    let body = res.text().await?;

    log::info!("{}", body);

    Ok(())
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("inside the create function");
        spawn_local(async{
            let test = get_test().await;
        });
        
        Self {test_string: "somethingsomething".to_string(), stl: css::home::styles()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg:Self::Message) -> bool {
        match msg {
            Msg::TestString(test_value) => {
                log::info!("inside update for Msg::TestString");
                log::info!("this is the value of test_value in update {:?}", test_value);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style={self.stl["main"].clone()}>
                <div style={self.stl["newspapertitle"].clone()} class="titleheader">
                    {"Transmission from the Outer Planets"}
                </div>
                <div>
                    <a href="/manipulation">{"A Course in the Dark Arts"}</a>
                </div>
                <div>
                    {self.view_info_tiles()}
                </div>
            </div>
        }
    }
}

impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "What are yews?" }</p>
                        <p class="subtitle">{ "Everything you need to know!" }</p>
                        <div>
                            {self.test_string.clone()}
                        </div>
                        <div class="content">
                            {r#"
                                A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
                                The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
                                The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
                                but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
                                except on erect leading shoots where the spiral arrangement is more obvious.
                                The leaves are poisonous.
                            "#}
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Who are we?" }</p>

                        <div class="content">
                            { "We're a small team of just 2" }
                            <sup>{ 64 }</sup>
                            { " members working tirelessly to bring you the low-effort yew content we all desperately crave." }
                            <br />
                            {r#"
                                We put a ton of effort into fact-checking our posts.
                                Some say they read like a Wikipedia article - what a compliment!
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
