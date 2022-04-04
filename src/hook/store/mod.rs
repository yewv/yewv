mod context;
mod handle;
mod store;

pub use context::*;
pub use handle::*;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
pub use store::*;
use yew::{hook, use_context, use_mut_ref, Hook, HookContext};

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
#[hook]
pub fn use_store<T: 'static>() -> UseStoreHandle<T> {
    let store_handle = UseStoreHandle::<T> {
        context: use_context().expect("Store context not registered"),
        subscriptions: use_mut_ref(|| Subscriptions {
            states: vec![],
            subscriptions: vec![],
            ref_subscriptions: vec![],
        }),
    };

    {
        let mut subs = store_handle.subscriptions.borrow_mut();
        subs.subscriptions.clear();
        subs.ref_subscriptions.clear();
    }

    use_store_subscription(&store_handle, {
        let subs = store_handle.subscriptions.clone();
        move |prev, next| {
            let mut subs = subs.borrow_mut();
            if !subs.subscriptions.is_empty() {
                let mut require_render = false;
                let mut next_states = vec![];
                for (i, sub) in subs.subscriptions.iter().enumerate() {
                    let state = subs
                        .states
                        .get(i)
                        .expect("Store subscription has no corresponding state.");
                    let next_state = sub(state.clone(), &next);
                    require_render |= !Rc::ptr_eq(&state, &next_state);
                    next_states.push(next_state);
                }
                subs.states = next_states;
                if require_render {
                    return true;
                }
            }
            for sub in subs.ref_subscriptions.iter() {
                if sub(Ref::clone(&prev), Ref::clone(&next)) {
                    return true;
                }
            }
            false
        }
    });

    store_handle
}

fn use_store_subscription<'a, T: 'static>(
    store: &'a Store<T>,
    callback: impl Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool + 'static,
) -> impl 'a + Hook<Output = ()> {
    struct WatchState(Rc<RefCell<bool>>);
    impl Drop for WatchState {
        fn drop(&mut self) {
            *self.0.borrow_mut() = false;
        }
    }
    struct HookProvider<'a, T: 'static, C: Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool> {
        store: &'a Store<T>,
        callback: C,
    }

    impl<'a, T: 'static, C: Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool + 'static> Hook
        for HookProvider<'a, T, C>
    {
        type Output = ();

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            ctx.next_state(|r| {
                let is_active = Rc::new(RefCell::new(true));
                let watch = WatchState(is_active.clone());
                self.store.subscribe(move |prev, next| {
                    if !*is_active.borrow() {
                        return false;
                    }
                    if (self.callback)(prev, next) {
                        (r)();
                    }
                    true
                });
                watch
            });
        }
    }

    HookProvider { store, callback }
}
