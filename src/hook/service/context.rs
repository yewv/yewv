use std::{ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct ServiceContext<T> {
    service: Rc<T>,
}

impl<T> ServiceContext<T> {
    pub fn new(service: T) -> Self {
        Self {
            service: Rc::new(service),
        }
    }
}

impl<T> PartialEq for ServiceContext<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.service, &other.service)
    }
}

impl<T> Clone for ServiceContext<T> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}

impl<T> Deref for ServiceContext<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}
