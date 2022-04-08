use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use yew::{function_component, ContextProvider};
use yewv::*;

pub struct StoreState {
    pub value: i32,
}

#[derive(PartialEq, Clone)]
pub enum SubscriptionType {
    Map,
    MapRef,
    Watch,
    WatchRef,
}

#[derive(Properties, PartialEq, Clone)]
pub struct StoreAppProps {
    pub sub_type: SubscriptionType,
    pub context: StoreContext<StoreState>,
    pub render_count: Rc<RefCell<i32>>,
}

impl StoreAppProps {
    pub fn new(sub_type: SubscriptionType) -> Self {
        Self {
            sub_type,
            context: StoreContext::new(StoreState { value: 0 }),
            render_count: Rc::new(RefCell::new(0)),
        }
    }
}

#[function_component(StoreApp)]
pub fn store_app(props: &StoreAppProps) -> Html {
    html! {
        <ContextProvider<StoreContext<StoreState>> context={props.context.clone()}>
        <div id={"result"}>
        {
            match &props.sub_type {
                SubscriptionType::Map => html! { <StoreMapComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::MapRef => html! { <StoreMapRefComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::Watch => html! { <StoreWatchComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::WatchRef => html! { <StoreWatchRefComponent render_count={props.render_count.clone()} /> },
            }
        }
        </div>
        </ContextProvider<StoreContext<StoreState>>>
    }
}

#[derive(Properties, PartialEq)]
struct StoreComponentProps {
    pub render_count: Rc<RefCell<i32>>,
}

#[function_component(StoreMapComponent)]
fn store_map_component(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    let value = store.map(|s| s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { value } }
}

#[function_component(StoreMapRefComponent)]
fn store_map_ref_component(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    let value = store.map_ref(|s| &s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { value } }
}

#[function_component(StoreWatchComponent)]
fn store_watch_component(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    store.watch(|s| s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { store.state().value } }
}

#[function_component(StoreWatchRefComponent)]
fn store_watch_ref_component(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    store.watch_ref(|s| &s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { store.state().value } }
}
