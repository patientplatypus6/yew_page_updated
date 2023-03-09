use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::router::BrowserRouter;

use crate::pages::home::Home;
use crate::pages::manipulation::Manipulation;
use crate::pages::canvas::Canvas;
use crate::pages::mdpages::Mdpages;
use gloo_net::http::Request;
use yew::{classes};
use std::collections::HashMap;


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/mdpages")]
    Mdpages,
    #[at("/canvas")]
    Canvas,
    #[at("/")]
    Home,
    #[at("/manipulation")]
    Manipulation,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {

    let mut style = HashMap::new();
    style.insert("main".to_string(), "background: rgba(0,0,0,0.2);");
    style.insert("nav_item".to_string(), "font-size: 20px;");

    html! {
        <BrowserRouter>
            <div> 
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { < Home /> },
        Route::Manipulation => html! { < Manipulation /> },
        Route::Canvas => html! { < Canvas />},
        Route::Mdpages => html! { <Mdpages /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

