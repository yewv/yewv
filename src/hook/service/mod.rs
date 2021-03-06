mod context;

pub use context::ServiceContext;
use yew::{hook, use_context};

/// Obtain a context for the given service `T`.
/// ```rust
/// use yew::prelude::*;
/// use yewv::use_service;
///
/// struct AppService { }
///
/// impl AppService {
///     pub fn trigger_some_action(&self) {
///         // Your custom logic
///     }
/// }
///
/// #[function_component]
/// fn Test() -> Html {
///     let service = use_service::<AppService>();
///     let onclick = move |_| service.trigger_some_action();
///     
///     html!{
///         <button {onclick}>{ "Trigger" }</button>
///     }
/// }
/// ```
#[hook]
pub fn use_service<T>() -> ServiceContext<T>
where
    T: 'static,
{
    use_context::<ServiceContext<T>>().expect("service was not registered.")
}
