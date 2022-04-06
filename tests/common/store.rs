use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use yew::{function_component, ContextProvider, Html};
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

#[function_component]
pub fn StoreApp(props: &StoreAppProps) -> Html {
    html! {
        <ContextProvider<StoreContext<StoreState>> context={props.context.clone()}>
        <div id={"result"}>
        {
            match &props.sub_type {
                SubscriptionType::Map => html! { <StoreMapRefComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::MapRef => html! { <StoreMapRefComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::Watch => html! { <StoreMapRefComponent render_count={props.render_count.clone()} /> },
                SubscriptionType::WatchRef => html! { <StoreMapRefComponent render_count={props.render_count.clone()} /> },
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

#[function_component]
fn StoreMapComponent(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    let value = store.map(|s| s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { value } }
}

#[function_component]
fn StoreMapRefComponent(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    let value = store.map_ref(|s| &s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { value } }
}

#[function_component]
fn StoreWatchComponent(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    store.watch(|s| s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { store.state().value } }
}

#[function_component]
fn StoreWatchRefComponent(props: &StoreComponentProps) -> Html {
    let store = use_store::<StoreState>();

    store.watch_ref(|s| &s.value);
    *props.render_count.borrow_mut() += 1;
    html! { { store.state().value } }
}
