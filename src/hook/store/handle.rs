use crate::{Store, StoreContext};
use std::{
    any::Any,
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

pub(crate) struct Subscriptions<T> {
    pub(crate) states: Vec<Rc<dyn Any>>,
    pub(crate) subscriptions: Vec<Box<dyn (Fn(Rc<dyn Any>, &T) -> Rc<dyn Any>)>>,
    pub(crate) ref_subscriptions: Vec<Box<dyn (Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool)>>,
}

/// Handle exposing custom hooks for the store.
pub struct UseStoreHandle<T: 'static> {
    pub(crate) context: StoreContext<T>,
    pub(crate) subscriptions: Rc<RefCell<Subscriptions<T>>>,
}

impl<T: 'static> UseStoreHandle<T> {
    /// (Hook) Subscribe to the store and return the value mapped.
    /// As opposed to `map_ref`, `watch` and `watch_ref`, `map` is a hook and is therefore constrained to certain rules:
    /// - Should only be called inside Yew function components.
    /// - Should not be called inside loops, conditions or nested functions.
    ///
    /// If you only wish to reference a value owned by the store, you should use `map_ref` instead.
    /// A change to the observed value will re-render the component.
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component]
    /// fn Test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     let value = store.map(|state| state.value);
    ///     
    ///     html!{ { value } }
    /// }
    /// ```
    pub fn map<M: PartialEq + 'static>(&self, map: impl Fn(&T) -> M + 'static) -> Rc<M> {
        let mut subscriptions = self.subscriptions.borrow_mut();
        let current_index = subscriptions.subscriptions.len();
        let value = match subscriptions.states.get(current_index) {
            Some(s) => s
                .clone()
                .downcast()
                .expect("Store map was called in a different order."),
            None => {
                let state = Rc::new(map(&self.state_ref()));
                subscriptions.states.push(state.clone());
                state
            }
        };
        subscriptions
            .subscriptions
            .push(Box::new(move |prev, next| {
                let next = map(next);
                let prev = prev
                    .downcast::<M>()
                    .expect("Store map was called in a different order.");
                if next.ne(&prev) {
                    return Rc::new(next);
                }
                prev
            }));
        value
    }

    /// Subscribe to the store and return a reference to the value mapped.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
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
    pub fn map_ref<'a, M: PartialEq + 'a>(&self, map: impl Fn(&Rc<T>) -> &M + 'static) -> Ref<M> {
        let value = Ref::map(self.state_ref(), &map);
        self.subscriptions
            .borrow_mut()
            .ref_subscriptions
            .push(Box::new(move |prev, next| {
                *Ref::map(prev, &map) != *Ref::map(next, &map)
            }));
        value
    }

    /// Subscribe to a specific store value.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component]
    /// fn Test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     store.watch_ref(|state| &state.value);
    ///     
    ///     html!{ { store.state().value } }
    /// }
    /// ```
    pub fn watch_ref<W: PartialEq>(&self, watch: impl Fn(&Rc<T>) -> &W + 'static) {
        self.subscriptions
            .borrow_mut()
            .ref_subscriptions
            .push(Box::new(move |prev, next| {
                *Ref::map(prev, &watch) != *Ref::map(next, &watch)
            }));
    }

    /// Subscribe to a specific store value.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component]
    /// fn Test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     store.watch(|state| state.value);
    ///     
    ///     html!{ { store.state().value } }
    /// }
    /// ```
    pub fn watch<W: PartialEq + 'static>(&self, watch: impl Fn(&T) -> W + 'static) {
        self.subscriptions
            .borrow_mut()
            .subscriptions
            .push(Box::new(move |prev, next| {
                let next = watch(next);
                let current = prev
                    .downcast::<W>()
                    .expect("Store hooks were called in a different order");
                if next.ne(&current) {
                    return Rc::new(next);
                }
                current
            }));
    }
}

impl<T> Deref for UseStoreHandle<T> {
    type Target = Rc<Store<T>>;

    fn deref(&self) -> &Self::Target {
        &self.context.store
    }
}
