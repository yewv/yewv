mod context;

pub use context::ServiceContext;
use yew::use_context;

pub fn use_service<T>() -> ServiceContext<T>
where
    T: 'static,
{
    use_context::<ServiceContext<T>>().expect("service was not registered.")
}
