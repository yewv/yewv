use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
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

    pub fn state_ref(&self) -> Ref<Rc<T>> {
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

    pub fn notify(&self) {
        let mut subs = self.subscriptions.borrow_mut().split_off(0);
        subs.retain(|s| s(self.previous_state.borrow(), self.state_ref()));
        self.subscriptions.borrow_mut().append(&mut subs);
    }

    pub fn subscribe(&self, callback: impl Fn(Ref<Rc<T>>, Ref<Rc<T>>) -> bool + 'static) {
        self.subscriptions.borrow_mut().push(Box::from(callback));
    }
}

impl<T> Debug for Store<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store").field("state", &self.state).finish()
    }
}
