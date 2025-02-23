bounce::use_query seems to stall forever without running the query under certain conditions.

To see, `trunk serve`, enter a non-zero number and press <kbd>Enter</kbd>

```rs
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
```


`use_query_value` on the otherhand works just fine.

There is a `use_query_value` version in [./src/app.rs](./src/app.rs). Uncomment to see it work.

