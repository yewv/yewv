# yewv
A lightning fast state management module for Yew built with performance and simplicity as a first priority.

# Install
```toml
[dependencies]
yewv = { git = "https://github.com/yewv/yewv" }
```
# Usage
## Simple app with store
```rust
// main.rs
use yew::prelude::*;
use yewv::*;

struct AppState {
    count: i32,
}

#[function_component(App)]
fn app() -> Html {
    let store = StoreContext::new(AppState { count: 0 });
    html! {
        <ContextProvider<StoreContext<AppState>> context={store} >
            <Counter />
            <Counter />
        </ContextProvider<StoreContext<AppState>>>
    }
}

#[function_component(Counter)]
fn counter() -> Html {
    let store = use_store::<AppState>();
    let count = store.map_ref(|state| &state.count);
    let onclick = {
        let store = store.clone();
        move |_| {
            let state = store.state();
            store.set_state(AppState {
                count: state.count + 1,
            });
        }
    };
    html! {
        <button {onclick}>{format!("{} +", count)}</button>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

## Simple app with store and service
```rust
use yew::prelude::*;
use yewv::*;

struct AppState {
    count: i32,
}

struct AppService {
    store: StoreContext<AppState>,
}

impl AppService {
    fn increment_count(&self) {
        let state = self.store.state();
        self.store.set_state(AppState {
            count: state.count + 1,
        });
    }
}

#[function_component(App)]
fn app() -> Html {
    let store = StoreContext::new(AppState { count: 0 });
    let service = ServiceContext::new(AppService {
        store: store.clone(),
    });
    html! {
        <ContextProvider<StoreContext<AppState>> context={store} >
        <ContextProvider<ServiceContext<AppService>> context={service} >
            <Counter />
            <Counter />
        </ContextProvider<ServiceContext<AppService>>>
        </ContextProvider<StoreContext<AppState>>>
    }
}

#[function_component(Counter)]
fn counter() -> Html {
    let service = use_service::<AppService>();
    let store = use_store::<AppState>();

    let count = store.map_ref(|state| &state.count);
    let onclick = move |_| service.increment_count();

    html! {
        <button {onclick}>{format!("{} +", count)}</button>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

## map vs map_ref
todo