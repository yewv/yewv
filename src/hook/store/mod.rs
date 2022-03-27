mod context;
mod store;

pub use context::*;
pub use store::*;
use yew::use_context;

pub fn use_store<T>() -> StoreContext<T> {
    use_context::<StoreContext<T>>().expect("Store context not registered")
}
