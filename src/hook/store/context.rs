use super::Store;
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    ops::Deref,
    rc::Rc,
};
use yew::{use_hook, use_mut_ref};

#[derive(Debug)]
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

    pub fn map<M: 'static>(&self, callback: impl Fn(&T) -> M + 'static) -> Rc<M>
    where
        M: PartialEq,
    {
        let state = use_mut_ref(|| Rc::new(callback(&self.store.state_ref())));
        let value = state.borrow().clone();
        use_store_sub(self.store.clone(), move |_, new_state| {
            let new_value = callback(&new_state);
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

    pub fn map_ref<'a, M: PartialEq + 'a>(&self, map: impl Fn(&Rc<T>) -> &M + 'static) -> Ref<M>
    where
        M: PartialEq,
    {
        let state = Ref::map(self.store.state_ref(), &map);
        self.watch(map);
        state
    }

    pub fn watch<W: PartialEq>(&self, watch: impl Fn(&Rc<T>) -> &W + 'static) {
        use_store_sub(self.store.clone(), move |old_state, new_state| {
            *Ref::map(old_state, &watch) != *Ref::map(new_state, &watch)
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
            if !*s.borrow() {
                *s.borrow_mut() = true;
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
