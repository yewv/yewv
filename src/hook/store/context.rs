use super::Store;
use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};
use yew::{use_hook, use_mut_ref};

/// Context which holds a reference to the store.
pub struct StoreContext<T>
where
    T: 'static,
{
    pub store: Rc<super::Store<T>>,
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
        let state = use_mut_ref(|| Rc::new(map(&self.store.state_ref())));
        let value = state.borrow().clone();
        use_store_sub(self.store.clone(), move |_, new_state| {
            let new_value = map(&new_state);
            let mut current_value = state.borrow_mut();
            if (**current_value).ne(&new_value) {
                *current_value = Rc::new(new_value);
                true
            } else {
                false
            }
        });
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
        let state = Ref::map(self.store.state_ref(), &map);
        self.watch_ref(map);
        state
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
        use_store_sub(self.store.clone(), move |old_state, new_state| {
            *Ref::map(old_state, &watch) != *Ref::map(new_state, &watch)
        });
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
        let state = use_mut_ref(|| watch(&self.store.state_ref()));
        use_store_sub(self.store.clone(), move |_, new_state| {
            let new_value = watch(&new_state);
            let mut current_value = state.borrow_mut();
            let has_changed = (*current_value).ne(&new_value);
            *current_value = new_value;
            has_changed
        });
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
        }
    }
}

fn use_store_sub<T>(store: Rc<Store<T>>, sub: impl Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool + 'static) {
    use_hook(
        || Rc::new(RefCell::new(false)),
        move |s, u| {
            let mut is_init = s.borrow_mut();
            if !*is_init {
                *is_init = true;
                let s = s.clone();
                store.subscribe(move |o, n| {
                    let is_active = *s.borrow();
                    if is_active {
                        if sub(o, n) {
                            u.callback(|_: &mut Rc<RefCell<bool>>| true);
                        }
                    }
                    is_active
                });
            }
            s.clone()
        },
        |s| {
            *s.borrow_mut() = false;
        },
    );
}
