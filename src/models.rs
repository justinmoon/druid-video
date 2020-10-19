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

use std::{io, sync::mpsc};

#[derive(Debug, Clone)]
pub struct Connection {
    connection: mpsc::Sender<Request>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.stop_stream();
    }
}

impl Connection {
    pub fn new(connection: mpsc::Sender<Request>) -> Self {
        Connection { connection }
    }

    pub fn start_stream(&self) {
        self.connection.send(Request::StartStream).unwrap();
    }

    pub fn stop_stream(&self) {
        self.connection.send(Request::StopStream).unwrap();
    }

    pub fn query_formats(&self) {
        self.connection.send(Request::QueryFormats).unwrap();
    }

    pub fn query_controls(&self) {
        self.connection.send(Request::QueryControls).unwrap();
    }

    pub fn format(&self) {
        self.connection.send(Request::GetFormat).unwrap();
    }

    pub fn set_format(&self, fmt: &Format) {
        self.connection
            .send(Request::SetFormat(fmt.clone()))
            .unwrap();
    }

    pub fn set_control(&self, ctrl: &Control) {
        self.connection
            .send(Request::SetControl(ctrl.clone()))
            .unwrap();
    }
}

#[derive(Debug)]
pub enum Request {
    StartStream,
    StopStream,
    QueryFormats,
    QueryControls,
    GetFormat,
    SetFormat(Format),
    SetControl(Control),
}

// FIXME: I don't think this will get used. We'll have druid commands instead.
#[derive(Debug)]
pub enum Response {
    StartStream(io::Result<()>),
    StopStream(io::Result<()>),
    QueryFormats(io::Result<Vec<Format>>),
    QueryControls(io::Result<Vec<Control>>),
    GetFormat(io::Result<Format>),
    SetFormat(io::Result<Format>),
    SetControl(io::Result<Control>),
}
