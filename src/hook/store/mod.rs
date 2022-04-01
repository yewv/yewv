mod context;
mod store;

pub use context::*;
pub use store::*;
use yew::use_context;

/// Obtain a store context for the given state `T`.
/// ```rust
/// use yew::prelude::*;
/// use yewv::use_store;
///
/// struct StoreState {
///     value: i32
/// }
///
/// #[function_component(Test)]
/// fn test() -> Html {
///     let store = use_store::<StoreState>();
///     let value = store.map_ref(|state| &state.value);
///     
///     html!{ { value } }
/// }
/// ```
pub fn use_store<T>() -> StoreContext<T> {
    use_context::<StoreContext<T>>().expect("Store context not registered")
}
