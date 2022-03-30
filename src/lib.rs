//! A lightning fast state management module for Yew built with performance and simplicity as a first priority.
//!
//! ## Who is this for?
//! If you wish to use a store alongside Yew fonction components, this library is made for you.
//!
//! ## Install
//! Add the following dependency to your `Cargo.toml`.
//! ```toml
//! [dependencies]
//! yewv = "0.1"
//! ```
//! ## Usage
//! The following need to be respected while using this library:
//! 1. Only works with Yew function components.
//! 2. Store and service contexts must be registered in a **parent** or **root** component with `ContextProvider`.
//! 3. Store and service need to be used in a **child** component with `use_store`/`use_service`.
//! ### Simple app with store
//! ```rust
//! // main.rs
//! use yew::prelude::*;
//! use yewv::*;
//!
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let store = StoreContext::new(AppState { count: 0 });
//!     html! {
//!         <ContextProvider<StoreContext<AppState>> context={store} >
//!             <Counter />
//!             <Counter />
//!         </ContextProvider<StoreContext<AppState>>>
//!     }
//! }
//!
//! #[function_component(Counter)]
//! fn counter() -> Html {
//!     let store = use_store::<AppState>();
//!     let count = store.map_ref(|state| &state.count);
//!     let onclick = {
//!         let store = store.clone();
//!         move |_| {
//!             let state = store.state();
//!             store.set_state(AppState {
//!                 count: state.count + 1,
//!             });
//!         }
//!     };
//!     html! {
//!         <button {onclick}>{format!("{} +", count)}</button>
//!     }
//! }
//!
//! fn main() {
//!     yew::start_app::<App>();
//! }
//! ```
//!
//! ### Simple app with store and service
//! ```rust
//! // main.rs
//! use yew::prelude::*;
//! use yewv::*;
//!
//! struct AppState {
//!     count: i32,
//! }
//!
//! struct AppService {
//!     store: StoreContext<AppState>,
//! }
//!
//! impl AppService {
//!     fn increment_count(&self) {
//!         let state = self.store.state();
//!         self.store.set_state(AppState {
//!             count: state.count + 1,
//!         });
//!     }
//! }
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let store = StoreContext::new(AppState { count: 0 });
//!     let service = ServiceContext::new(AppService {
//!         store: store.clone(),
//!     });
//!     html! {
//!         <ContextProvider<StoreContext<AppState>> context={store} >
//!         <ContextProvider<ServiceContext<AppService>> context={service} >
//!             <Counter />
//!             <Counter />
//!         </ContextProvider<ServiceContext<AppService>>>
//!         </ContextProvider<StoreContext<AppState>>>
//!     }
//! }
//!
//! #[function_component(Counter)]
//! fn counter() -> Html {
//!     let service = use_service::<AppService>();
//!     let store = use_store::<AppState>();
//!
//!     let count = store.map_ref(|state| &state.count);
//!     let onclick = move |_| service.increment_count();
//!
//!     html! {
//!         <button {onclick}>{format!("{} +", count)}</button>
//!     }
//! }
//!
//! fn main() {
//!     yew::start_app::<App>();
//! }
//! ```
//!
//! ### map vs map_ref
//! If you only wish to reference a value owned by the store, you should use `map_ref`.
//! As opposed to `map`, `map_ref` doesn't take ownership of the referenced value.
//! It is usually preferable to use `map_ref` over `map` when possible.
//! However, it is not always possible to use `map_ref`. For instance, if the value you wish to access is not owned by the store state, you will need to use `map`:
//! ```rust
//! let mapped = store.map(|state| state.some_vector.len());
//! ```
//!
//! ## Why is it so fast?
//! ### Custom hooks
//! The store utilizes highly optimized custom hooks for better performance and memory efficiency.
//! ### Renders only when and where needed
//! Subscriptions done to the store with `map`, `map_ref` and `watch` will only trigger a render on the component if the observed value has changed.
//! ### Reference VS Ownership
//! Instead of propagating clone/copy of the application state throughout components, references are used.
//!
//! ## Good practices
//! ### Reference only what's needed
//! When you are observing a value in a store, make sure you are not taking more than necessary. For instance, if you are only interested in a single value from a vector, there is no need to reference the entire vector:
//! ```rust
//! let mapped = store.map_ref(|state| &state.some_vector[0]);
//! let another = store.map_ref(|state| state.some_vector.iter().last().expect("to have a value"));
//! ```
//!
//! ### Segregation of stores in large applications
//! When and where it makes sense, try to break your monolithic stores into multiple. Doing so will improve the performance of the application as a whole.
//!
//! ## Credits
//! - [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)
//! - [Yew](https://github.com/yewstack/yew) - [MIT](https://github.com/yewstack/yew/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/yewstack/yew/blob/master/LICENSE-APACHE)

mod hook;
pub use hook::*;
