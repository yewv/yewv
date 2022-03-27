use super::Store;
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    ops::Deref,
    rc::Rc,
};
use yew::{use_effect_with_deps, use_hook, HookUpdater};

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
        let state = use_opt_state_eq(|| callback(&self.store.state_ref()));
        let value = state.0.clone();
        use_effect_with_deps(
            {
                let store = self.store.clone();
                move |_| {
                    let is_active = Rc::new(RefCell::new(true));
                    store.subscribe({
                        let is_active = is_active.clone();
                        move |_, new_state| {
                            if !*is_active.borrow() {
                                return false;
                            }
                            state.set(callback(&new_state));
                            true
                        }
                    });
                    move || {
                        *is_active.borrow_mut() = false;
                    }
                }
            },
            (),
        );
        value
    }

    pub fn map_ref<'a, M: 'a>(&self, callback: impl Fn(&Rc<T>) -> &M + 'static) -> Ref<M>
    where
        M: PartialEq,
    {
        let renderer = use_renderer();
        let state = Ref::map(self.store.state_ref(), &callback);
        use_effect_with_deps(
            {
                let store = self.store.clone();
                move |_| {
                    let is_active = Rc::new(RefCell::new(true));
                    store.subscribe({
                        let is_active = is_active.clone();
                        move |old_state, new_state| {
                            if !*is_active.borrow() {
                                return false;
                            }
                            if *Ref::map(old_state, &callback) != *Ref::map(new_state, &callback) {
                                renderer.render();
                            }
                            true
                        }
                    });
                    move || {
                        *is_active.borrow_mut() = false;
                    }
                }
            },
            (),
        );
        state
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

struct Renderer(Option<HookUpdater>);

impl Renderer {
    pub fn render(&self) {
        self.0
            .as_ref()
            .expect("HookUpdater should be initialized")
            .callback(|_: &mut Renderer| true);
    }
}

fn use_renderer() -> Renderer {
    use_hook(
        || Renderer(None),
        |_: &mut Renderer, u| Renderer(Some(u)),
        |_| {},
    )
}

struct State<T: PartialEq + 'static>(Rc<T>, Option<HookUpdater>);

impl<T: PartialEq + 'static> State<T> {
    pub fn set(&self, state: T) {
        if state.ne(&self.0) {
            self.1
                .as_ref()
                .expect("HookUpdater should be initialized.")
                .callback(|s: &mut State<T>| {
                    let should_render = state.ne(&s.0);
                    s.0 = Rc::new(state);
                    should_render
                })
        }
    }
}

fn use_opt_state_eq<T: PartialEq + 'static>(init: impl FnOnce() -> T) -> State<T> {
    use_hook(
        || State(Rc::new(init()), None),
        |s: &mut State<T>, u| State(s.0.clone(), Some(u)),
        |_| {},
    )
}
