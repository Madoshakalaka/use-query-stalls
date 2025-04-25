use std::{convert::Infallible, rc::Rc};

use async_trait::async_trait;
use bounce::{
    prelude::*,
    query::{use_query, use_query_value, Query, QueryResult},
    BounceRoot,
};
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(PartialEq)]
struct SearchQuery(String);

impl Query for SearchQuery {
    type Input = usize;
    type Error = Infallible;

    async fn query(_states: &BounceStates, limit: Rc<usize>) -> QueryResult<Self> {
        if *limit == 0 {
            return Ok(SearchQuery("default query result".to_string()).into());
        }

        gloo_console::info!(format!("presenting you {limit} dog facts!"));
        let url = "https://dogapi.dog/api/v2/facts";
        let resp = Request::get(url)
            .query([("limit", limit.to_string())])
            .send()
            .await
            .expect("failed to fetch");

        let body = resp.text().await.expect("failed to read body");
        gloo_console::info!(format!("body: {body}"));
        Ok(SearchQuery(body).into())
    }
}

// #[function_component]
// pub fn Search() -> Html {
//     let search_state = use_state(|| 0);
//
//     gloo_console::info!(format!("search state: {search_state:?}"));
//     let search_results = use_query_value::<SearchQuery>((*search_state).into());
//
//     let onkeydown = use_callback((), {
//         let search_state = search_state.setter();
//         move |e: KeyboardEvent, _| {
//             if e.key() == "Enter" {
//                 let value = e
//                     .target_dyn_into::<web_sys::HtmlInputElement>()
//                     .unwrap()
//                     .value();
//                 search_state.set(value.parse().unwrap());
//             }
//         }
//     });
//
//     html! {
//         <div>
//                 <input type="text" placeholder="How many dog facts?" {onkeydown} />
//                 if let Some(Ok(results)) = search_results.result() {
//                     <div>{ results.0.clone() }</div>
//                 } else {
//                     <div>{ "Searching or Errored" }</div>
//                 }
//         </div>
//     }
// }

#[function_component]
pub fn Search() -> HtmlResult {
    let search_state = use_state(|| 0);

    gloo_console::info!(format!("search state: {search_state:?}"));
    let search_results = use_query::<SearchQuery>((*search_state).into())?;

    let onkeydown = use_callback((), {
        let search_state = search_state.setter();
        move |e: KeyboardEvent, _| {
            if e.key() == "Enter" {
                let value = e
                    .target_dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();
                search_state.set(value.parse().unwrap());
            }
        }
    });

    Ok(html! {
        <div>
            <input type="text" placeholder="How many dog facts?" {onkeydown} />
            if let Ok(results) = &*search_results {
                <div>{ results.0.clone() }</div>
            } else {
                <div>{ "Errored" }</div>
            }
        </div>
    })
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BounceRoot>
            <Suspense fallback={"fetching... (actually stalling...)"}>
                <Search/>
            </Suspense>
        </BounceRoot>
    }
}
