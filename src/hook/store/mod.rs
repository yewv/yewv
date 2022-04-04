mod context;
mod store;

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub use context::*;
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
pub fn use_store<T: 'static>() -> StoreContext<T> {
    let mut context = use_context::<StoreContext<T>>().expect("Store context not registered");

    //TODO: Should only be one hook.
    context.states = use_mut_ref(|| vec![]);
    context.subscriptions = use_mut_ref(|| vec![]);
    context.subscriptions.borrow_mut().clear();
    context.ref_subscriptions = use_mut_ref(|| vec![]);
    context.ref_subscriptions.borrow_mut().clear();

    use_store_subscription(&context, {
        let states = context.states.clone();
        let subscriptions = context.subscriptions.clone();
        let ref_subscriptions = context.ref_subscriptions.clone();
        move |prev, next| {
            let mut states = states.borrow_mut();
            let mut require_render = false;
            for (i, sub) in subscriptions.borrow().iter().enumerate() {
                let state = states
                    .get(i)
                    .expect("Store subscription has no corresponding state.");
                let next_state = sub(state.clone(), &next);
                if !Rc::ptr_eq(state, &next_state) {
                    require_render |= true;
                    states[i] = next_state;
                }
            }
            if require_render {
                return true;
            }
            for sub in ref_subscriptions.borrow().iter() {
                if sub(Ref::clone(&prev), Ref::clone(&next)) {
                    return true;
                }
            }
            false
        }
    });

    context
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
