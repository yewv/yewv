mod store;
use std::time::Duration;

use gloo::timers::future::sleep;
pub use store::*;
use yew::prelude::*;

pub async fn render_with_props<C: Component + 'static>(props: <C as Component>::Properties) {
    yew::start_app_with_props_in_element::<C>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
        props,
    );
    sleep(Duration::ZERO).await;
}

pub async fn inner_html() -> String {
    sleep(Duration::ZERO).await;
    gloo_utils::document()
        .get_element_by_id("result")
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

pub async fn wait() {
    sleep(Duration::ZERO).await;
}
