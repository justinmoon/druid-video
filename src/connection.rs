use std::{io, sync::mpsc};

use crate::models;

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

    pub fn set_format(&self, fmt: &models::Format) {
        self.connection
            .send(Request::SetFormat(fmt.clone()))
            .unwrap();
    }

    pub fn set_control(&self, ctrl: &models::Control) {
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
    SetFormat(models::Format),
    SetControl(models::Control),
}

#[derive(Debug)]
pub enum Response {
    StartStream(io::Result<()>),
    StopStream(io::Result<()>),
    QueryFormats(io::Result<Vec<models::Format>>),
    QueryControls(io::Result<Vec<models::Control>>),
    GetFormat(io::Result<models::Format>),
    SetFormat(io::Result<models::Format>),
    SetControl(io::Result<models::Control>),
}
