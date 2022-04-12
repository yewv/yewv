mod context;
mod handle;
mod store;

pub use context::*;
use handle::*;
use std::{cell::RefCell, rc::Rc};
pub use store::*;
use yew::{use_context, use_hook};

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
pub fn use_store<T>() -> handle::UseStoreHandle<T> {
    let context = use_context::<StoreContext<T>>().expect("Store context not registered");

    let subscriptions = use_hook(
        || {
            (
                Rc::new(RefCell::new(Subscriptions::<T> {
                    states: vec![],
                    subscriptions: vec![],
                    ref_subscriptions: vec![],
                })),
                Rc::new(RefCell::new(false)),
            )
        },
        {
            let store = context.store.clone();
            move |x: &mut (Rc<RefCell<Subscriptions<T>>>, Rc<RefCell<bool>>), u| {
                let mut is_active = x.1.borrow_mut();
                if !*is_active {
                    *is_active = true;
                    store.subscribe({
                        let subs = x.0.clone();
                        let is_active = x.1.clone();
                        move |prev, next| {
                            if !*is_active.borrow() {
                                return false;
                            }
                            let mut require_render = false;
                            {
                                let mut subs = subs.borrow_mut();
                                if subs.subscriptions.len() > 0 {
                                    let mut next_states = std::mem::take(&mut subs.states);
                                    for (i, sub) in subs.subscriptions.iter().enumerate() {
                                        let state = next_states.get_mut(i).expect(
                                            "Store subscription has no corresponding state.",
                                        );
                                        let next_state = sub(state.clone(), &next);
                                        require_render = !Rc::ptr_eq(&state, &next_state);
                                        *state = next_state;
                                    }
                                    subs.states = next_states;
                                }
                                if !require_render {
                                    for sub in subs.ref_subscriptions.iter() {
                                        if sub(prev, next) {
                                            require_render = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if require_render {
                                u.callback(
                                    |_: &mut (Rc<RefCell<Subscriptions<T>>>, Rc<RefCell<bool>>)| {
                                        true
                                    },
                                );
                            }
                            true
                        }
                    });
                }
                (x.0.clone(), x.1.clone())
            }
        },
        |x| *x.1.borrow_mut() = false,
    )
    .0;
    {
        let mut subs = subscriptions.borrow_mut();
        subs.subscriptions.clear();
        subs.ref_subscriptions.clear();
    }

    handle::UseStoreHandle {
        context,
        subscriptions,
    }
}
