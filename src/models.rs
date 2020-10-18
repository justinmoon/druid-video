#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Device {
    pub uri: String,
}

impl core::convert::From<&str> for Device {
    fn from(uri: &str) -> Self {
        Device {
            uri: uri.to_string(),
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

use eye::control::Control as Control_;

#[derive(Clone)]
pub struct Control {
    pub id: u32,
    pub name: String,

    pub representation: Representation,
    pub value: Value,
}

impl core::convert::From<&Control_> for Control {
    fn from(ctrl: &Control_) -> Self {
        Control {
            id: ctrl.id,
            name: ctrl.name.clone(),
            representation: ctrl.repr.clone(),
            value: Value::None,
        }
    }
}

impl std::fmt::Debug for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Control")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub type Representation = eye::control::Representation;
pub type Value = eye::control::Value;

use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct SendWrapper<T> {
    inner: T,
}

impl<T> SendWrapper<T> {
    pub unsafe fn new(val: T) -> Self {
        SendWrapper { inner: val }
    }
}

impl<T> Deref for SendWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SendWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl<T> Send for SendWrapper<T> {}
