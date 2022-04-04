use super::Store;
use std::{
    any::Any,
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

/// Context which holds a reference to the store.
pub struct StoreContext<T>
where
    T: 'static,
{
    pub store: Rc<super::Store<T>>,
    pub(crate) states: Rc<RefCell<Vec<Rc<dyn Any>>>>,
    pub(crate) subscriptions: Rc<RefCell<Vec<Box<dyn (Fn(Rc<dyn Any>, &T) -> Rc<dyn Any>)>>>>,
    pub(crate) ref_subscriptions: Rc<RefCell<Vec<Box<dyn (Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool)>>>>,
}

impl<T> PartialEq for StoreContext<T>
where
    T: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

impl<T> StoreContext<T>
where
    T: 'static,
{
    pub fn new(initial_state: T) -> Self {
        Self {
            store: Rc::new(Store::new(initial_state)),
            states: Rc::new(RefCell::new(vec![])),
            subscriptions: Rc::new(RefCell::new(vec![])),
            ref_subscriptions: Rc::new(RefCell::new(vec![])),
        }
    }

    /// (Hook) Subscribe to the store and return the value mapped.
    /// If you only wish to reference a value owned by the store, you should use `map_ref` instead.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component(Test)]
    /// fn test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     let value = store.map(|state| state.value);
    ///     
    ///     html!{ { value } }
    /// }
    /// ```
    pub fn map<M: PartialEq + 'static>(&self, map: impl Fn(&T) -> M + 'static) -> Rc<M> {
        let mut subscriptions = self.subscriptions.borrow_mut();
        let current_index = subscriptions.len();
        let mut states = self.states.borrow_mut();
        let value = match states.get(current_index) {
            Some(s) => s
                .clone()
                .downcast()
                .expect("Store hooks were called in a different order."),
            None => {
                let state = Rc::new(map(&self.store.state_ref()));
                states.push(state.clone());
                state
            }
        };
        subscriptions.push(Box::new(move |prev, next| {
            let next = map(next);
            let prev = prev
                .downcast::<M>()
                .expect("Store hooks were called in a different order.");
            if next.ne(&prev) {
                return Rc::new(next);
            }
            prev
        }));
        value
    }

    /// (Hook) Subscribe to the store and return a reference to the value mapped.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
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
    pub fn map_ref<'a, M: PartialEq + 'a>(&self, map: impl Fn(&Rc<T>) -> &M + 'static) -> Ref<M> {
        let value = Ref::map(self.state_ref(), &map);
        self.ref_subscriptions
            .borrow_mut()
            .push(Box::new(move |prev, next| {
                *Ref::map(prev, &map) != *Ref::map(next, &map)
            }));
        value
    }

    /// (Hook) Subscribe to a specific store value.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component(Test)]
    /// fn test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     store.watch_ref(|state| &state.value);
    ///     
    ///     html!{ { store.state().value } }
    /// }
    /// ```
    pub fn watch_ref<W: PartialEq>(&self, watch: impl Fn(&Rc<T>) -> &W + 'static) {
        self.ref_subscriptions
            .borrow_mut()
            .push(Box::new(move |prev, next| {
                *Ref::map(prev, &watch) != *Ref::map(next, &watch)
            }));
    }

    /// (Hook) Subscribe to a specific store value.
    /// A change to the observed value will re-render the component.
    /// ```rust
    /// use yew::prelude::*;
    /// use yewv::*;
    ///
    /// struct StoreState {
    ///     value: i32
    /// }
    ///
    /// #[function_component(Test)]
    /// fn test() -> Html {
    ///     let store = use_store::<StoreState>();
    ///     store.watch(|state| state.value);
    ///     
    ///     html!{ { store.state().value } }
    /// }
    /// ```
    pub fn watch<W: PartialEq + 'static>(&self, watch: impl Fn(&T) -> W + 'static) {
        self.subscriptions
            .borrow_mut()
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

impl<T> Deref for StoreContext<T> {
    type Target = Store<T>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<T> Clone for StoreContext<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            states: Rc::new(RefCell::new(vec![])),
            subscriptions: Rc::new(RefCell::new(vec![])),
            ref_subscriptions: Rc::new(RefCell::new(vec![])),
        }
    }
}
