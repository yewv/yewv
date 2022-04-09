use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

/// Simple store with subscription capability.
pub struct Store<T> {
    previous_state: RefCell<Rc<T>>,
    state: RefCell<Rc<T>>,
    subscriptions: RefCell<Vec<Box<dyn Fn(&T, &T) -> bool>>>,
}

impl<T> Store<T> {
    /// Create a new instance of a store with the given state as initial state.
    /// ```rust
    /// use yewv::Store;
    ///
    /// let store = Store::new(0);
    /// assert_eq!(*store.state(), 0);
    /// ```
    pub fn new(initial_state: T) -> Self {
        let state = Rc::new(initial_state);
        Self {
            previous_state: RefCell::new(state.clone()),
            state: RefCell::new(state),
            subscriptions: RefCell::new(vec![]),
        }
    }

    /// Give a reference to the current store state.
    /// ```rust
    /// use yewv::Store;
    ///
    /// let store = Store::new(0);
    /// assert_eq!(*store.state(), 0);
    /// store.set_state(1);
    /// assert_eq!(*store.state(), 1);
    /// ```
    pub fn state(&self) -> Rc<T> {
        self.state.borrow().clone()
    }

    /// Set store next state.
    /// ```rust
    /// use yewv::Store;
    ///
    /// let store = Store::new(0);
    /// assert_eq!(*store.state(), 0);
    /// store.set_state(1);
    /// assert_eq!(*store.state(), 1);
    /// ```
    pub fn set_state(&self, new_state: T) {
        {
            let mut state = self.state.borrow_mut();
            *self.previous_state.borrow_mut() = state.clone();
            *state = Rc::new(new_state);
        }
        self.notify();
    }

    /// Subscibe to changes made to the store state.
    /// Your subscription will stay active as long as your `callback` returns `true`.
    /// When the `callback` returns `false` the subscription will be dropped.
    /// ```rust
    /// use yewv::Store;
    ///
    /// let store = Store::new(0);
    /// store.subscribe(|prev_state, current_state| {
    ///     /* Put your own subscription logic. */
    ///     true // Should be the condition for unsubscription.
    /// } );
    /// ```
    pub fn subscribe(&self, callback: impl Fn(&T, &T) -> bool + 'static) {
        self.subscriptions.borrow_mut().push(Box::from(callback));
    }

    pub(crate) fn notify(&self) {
        let mut subs = std::mem::take(&mut *self.subscriptions.borrow_mut());
        let previous = &self.previous_state.borrow();
        let next = &self.state_ref();
        subs.retain(|s| s(previous, next));
        self.subscriptions.borrow_mut().append(&mut subs);
    }

    pub(crate) fn state_ref(&self) -> Ref<Rc<T>> {
        self.state.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext<T> {
        notified_values: Rc<RefCell<Vec<(T, T)>>>,
        is_sub_active: Rc<RefCell<bool>>,
        store: Store<T>,
    }

    fn setup<T: Clone + 'static>(initial_state: T) -> TestContext<T> {
        let store = Store::new(initial_state);
        let notified_values = Rc::new(RefCell::new(vec![]));
        let is_sub_active = Rc::new(RefCell::new(true));
        store.subscribe({
            let notified_values = notified_values.clone();
            let is_sub_active = is_sub_active.clone();
            move |prev, next| {
                notified_values
                    .borrow_mut()
                    .push((prev.clone(), next.clone()));
                *is_sub_active.borrow()
            }
        });
        TestContext {
            notified_values,
            is_sub_active,
            store,
        }
    }

    #[test]
    fn set_state_with_new_state_should_update_current_state() {
        //Given
        let ctx = setup(0);
        //When
        ctx.store.set_state(1);
        //Then
        assert_eq!(*ctx.store.state(), 1);
    }

    #[test]
    fn set_state_with_new_state_should_update_previous_state() {
        //Given
        let ctx = setup(0);
        ctx.store.set_state(1);
        //When
        ctx.store.set_state(2);
        //Then
        assert_eq!(**ctx.store.previous_state.borrow(), 1);
    }

    #[test]
    fn set_state_with_new_state_should_notify() {
        //Given
        let ctx = setup(0);
        //When
        ctx.store.set_state(1);
        //Then
        assert_eq!(*ctx.notified_values.borrow(), &[(0, 1)]);
    }

    #[test]
    fn set_state_with_subscription_no_longer_active_should_no_longer_notify() {
        //Given
        let ctx = setup(0);
        *ctx.is_sub_active.borrow_mut() = false;
        ctx.store.set_state(1);
        let notify_count = ctx.notified_values.borrow().len();
        //When
        ctx.store.set_state(2);
        //Then
        assert_eq!(ctx.notified_values.borrow().len(), notify_count);
    }

    #[test]
    fn set_state_with_subscription_no_longer_active_should_drop_subscription() {
        //Given
        let ctx = setup(0);
        let sub_count = ctx.store.subscriptions.borrow().len();
        *ctx.is_sub_active.borrow_mut() = false;
        ctx.store.set_state(1);
        //When
        ctx.store.set_state(2);
        //Then
        assert_eq!(ctx.store.subscriptions.borrow().len(), sub_count - 1);
    }

    #[test]
    fn subscribe_with_callback_should_add_callback_to_subscriptions() {
        //Given
        let ctx = setup(0);
        let sub_count = ctx.store.subscriptions.borrow().len();
        //When
        ctx.store.subscribe(|_, _| false);
        //Then
        assert_eq!(ctx.store.subscriptions.borrow().len(), sub_count + 1);
    }
}
