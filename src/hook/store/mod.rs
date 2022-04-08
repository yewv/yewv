mod context;
mod handle;
mod store;

pub use context::*;
pub use handle::*;
use std::{
    any::Any,
    cell::{Ref, RefCell},
    rc::Rc,
};
pub use store::*;
use yew::{hook, html::AnyScope, use_context, Hook, HookContext};

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
    let subscriptions = use_store_subscription(&context);
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

fn use_store_subscription<'a, T: 'static>(
    store: &'a Store<T>,
) -> impl 'a + Hook<Output = Rc<RefCell<Subscriptions<T>>>> {
    struct WatchState(Rc<RefCell<bool>>);
    impl Drop for WatchState {
        fn drop(&mut self) {
            *self.0.borrow_mut() = false;
        }
    }
    struct HookProvider<'a, T: 'static> {
        store: &'a Store<T>,
    }

    impl<'a, T: 'static> Hook for HookProvider<'a, T> {
        type Output = Rc<RefCell<Subscriptions<T>>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            // HACK: It is way faster to implement our own hook (~2x more efficient).
            let ctx: &mut MyHookContext = unsafe { std::mem::transmute(ctx) };
            ctx.next_state(|r| {
                let is_active = Rc::new(RefCell::new(true));
                let watch = WatchState(is_active.clone());
                let subs = Rc::new(RefCell::new(Subscriptions::<T> {
                    states: vec![],
                    subscriptions: vec![],
                    ref_subscriptions: vec![],
                }));
                self.store.subscribe({
                    let subs = subs.clone();
                    move |prev, next| {
                        if !*is_active.borrow() {
                            return false;
                        }
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
                                (r)();
                                return true;
                            }
                        }
                        for sub in subs.ref_subscriptions.iter() {
                            if sub(Ref::clone(&prev), Ref::clone(&next)) {
                                (r)();
                                return true;
                            }
                        }
                        true
                    }
                });
                (subs, watch)
            })
            .0
            .clone()
        }
    }

    HookProvider { store }
}

/// HACK: Clone of yew::functional::HookContext for transumation.
#[allow(dead_code)]
struct MyHookContext {
    scope: AnyScope,
    re_render: Rc<dyn Fn()>,

    states: Vec<Rc<dyn Any>>,
    effects: Vec<Rc<dyn Any>>,

    counter: usize,
}

/// HACK: Clone of yew::functional::HookContext for transumation.
impl MyHookContext {
    fn next_state<T>(&mut self, initializer: impl FnOnce(Rc<dyn Fn()>) -> T) -> Rc<T>
    where
        T: 'static,
    {
        // Determine which hook position we're at and increment for the next hook
        let hook_pos = self.counter;
        self.counter += 1;

        match self.states.get(hook_pos) {
            Some(m) => m.clone().downcast().unwrap(),
            None => {
                let initial_state = Rc::new(initializer(self.re_render.clone()));
                self.states.push(initial_state.clone());

                initial_state
            }
        }
    }
}
