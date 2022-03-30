use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub struct Store<T> {
    previous_state: RefCell<Rc<T>>,
    state: RefCell<Rc<T>>,
    subscriptions: RefCell<Vec<Box<dyn Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool>>>,
}

impl<T> Store<T> {
    pub fn new(initial_state: T) -> Self {
        let state = Rc::new(initial_state);
        Self {
            previous_state: RefCell::new(state.clone()),
            state: RefCell::new(state),
            subscriptions: RefCell::new(vec![]),
        }
    }

    pub fn state(&self) -> Rc<T> {
        self.state.borrow().clone()
    }

    pub(crate) fn state_ref(&self) -> Ref<Rc<T>> {
        self.state.borrow()
    }

    pub fn set_state(&self, new_state: T) {
        {
            let mut state = self.state.borrow_mut();
            *self.previous_state.borrow_mut() = state.clone();
            *state = Rc::new(new_state);
        }
        self.notify();
    }

    pub(crate) fn notify(&self) {
        let mut subs = self.subscriptions.borrow_mut().split_off(0);
        subs.retain(|s| s(self.previous_state.borrow(), self.state_ref()));
        self.subscriptions.borrow_mut().append(&mut subs);
    }

    pub fn subscribe(&self, callback: impl Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool + 'static) {
        self.subscriptions.borrow_mut().push(Box::from(callback));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext<T> {
        notified_values: Rc<RefCell<Vec<(Rc<T>, Rc<T>)>>>,
        is_sub_active: Rc<RefCell<bool>>,
        store: Store<T>,
    }

    fn setup<T: 'static>(initial_state: T) -> TestContext<T> {
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
        assert_eq!(*ctx.notified_values.borrow(), &[(Rc::new(0), Rc::new(1))]);
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
        //Arrange
        let ctx = setup(0);
        let sub_count = ctx.store.subscriptions.borrow().len();
        //Act
        ctx.store.subscribe(|_, _| false);
        //Assert
        assert_eq!(ctx.store.subscriptions.borrow().len(), sub_count + 1);
    }
}
