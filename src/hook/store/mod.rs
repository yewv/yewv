mod context;
mod handle;
mod store;

pub use context::*;
pub use handle::*;
use std::{cell::RefCell, rc::Rc};
pub use store::*;
use yew::{hook, use_context, use_force_update, use_state};

/// Obtain a store context for the given state `T`.
/// ```rust
/// use yew::prelude::*;
/// use yewv::use_store;
///
/// struct StoreState {
///     value: i32
/// }
///
/// #[function_component]
/// fn Test() -> Html {
///     let store = use_store::<StoreState>();
///     let value = store.map_ref(|state| &state.value);
///     
///     html!{ { value } }
/// }
/// ```
#[hook]
pub fn use_store<T: 'static>() -> UseStoreHandle<T> {
    let context = use_context::<StoreContext<T>>().expect("Store context not registered");
    let renderer = use_force_update();
    // use_state is use because it is the most efficient hook to hold a state in Yew 0.20.
    // Another way to be ~5% more efficient would be to implement our own hook unsafely.
    // However, the difference is not significant enought to justify the use of unsafe.
    let subscriptions = use_state({
        let store = context.store.clone();
        move || {
            let is_active = Rc::new(RefCell::new(true));
            let watch = WatchState(is_active.clone());
            let subs = Rc::new(RefCell::new(Subscriptions::<T> {
                states: vec![],
                subscriptions: vec![],
                ref_subscriptions: vec![],
            }));
            store.subscribe({
                let subs = subs.clone();
                move |prev, next| {
                    if !*is_active.borrow() {
                        return false;
                    }
                    let mut subs = subs.borrow_mut();
                    if !subs.subscriptions.is_empty() {
                        let mut require_render = false;
                        let mut next_states = std::mem::take(&mut subs.states);
                        for (i, sub) in subs.subscriptions.iter().enumerate() {
                            let state = next_states
                                .get_mut(i)
                                .expect("Store subscription has no corresponding state.");
                            let next_state = sub(state.clone(), &next);
                            require_render |= !Rc::ptr_eq(&state, &next_state);
                            *state = next_state
                        }
                        subs.states = next_states;
                        if require_render {
                            renderer.force_update();
                            return true;
                        }
                    }
                    for sub in subs.ref_subscriptions.iter() {
                        if sub(prev, next) {
                            renderer.force_update();
                            return true;
                        }
                    }
                    true
                }
            });
            (subs, watch)
        }
    })
    .0
    .clone();
    {
        let mut subs = subscriptions.borrow_mut();
        subs.subscriptions.clear();
        subs.ref_subscriptions.clear();
    }

    UseStoreHandle {
        context,
        subscriptions,
    }
}

struct WatchState(Rc<RefCell<bool>>);
impl Drop for WatchState {
    fn drop(&mut self) {
        *self.0.borrow_mut() = false;
    }
}
