mod immediate;
mod models;

use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};

use druid::widget::{Button, FillStrat, Flex, Image, ImageData, SizedBox};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, LocalizedString,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};
use eye::hal::traits::{Device as DeviceTrait, Stream};

use eye::prelude::*;
use ffimage::packed::dynamic::{ImageBuffer, ImageView};

use crate::models::{Connection, Request, SendWrapper};

const IMAGE_DATA: Selector<ImageData> = Selector::new("IMAGE_DATA");
const START: Selector<URI> = Selector::new("START");
const STOP: Selector = Selector::new("STOP");

struct URI(String);

struct Delegate {}

#[derive(Clone, Data, Lens)]
struct AppState {
    connection: Arc<models::Connection>,
    buffer: Option<Arc<ImageData>>,
}

impl AppState {
    fn new(connection: models::Connection) -> Self {
        Self {
            buffer: None,
            connection: Arc::new(connection),
        }
    }
}

// Grab the first device
fn get_uri() -> Option<URI> {
    let devices: Vec<models::Device> = eye::device::Device::enumerate()
        .iter()
        .map(|dev| models::Device::from(dev.as_str()))
        .collect();

    if devices.len() > 0 {
        return Some(URI(devices[0].uri.clone()));
    }
    return None;
}

fn open_device(uri: &URI) -> Box<dyn DeviceTrait> {
    // Grab the device
    let mut device = eye::device::Device::with_uri(&uri.0).expect("with_uri() error");

    // Set the format
    let mut format = device.format().expect("couldn't read format");
    format.pixfmt = PixelFormat::Bgra(32);
    let format = device.set_format(&format).expect("Couldn't set format");
    if format.pixfmt != PixelFormat::Bgra(32) {
        panic!("device does not support BGRA capture",);
    }

    return device;
}

//enum CamThread<'a> {
//Idle {
//sink: ExtEventSink,
//receiver: Receiver<Request>,
////device: SendWrapper<Box<dyn Device>>,
//},
//Streaming {
//sink: ExtEventSink,
//receiver: Receiver<Request>,
//device: SendWrapper<Box<dyn DeviceTrait>>,
//stream: SendWrapper<Box<dyn 'a + for<'b> Stream<'b, Item = ImageView<'b>>>>,
//},
//}

struct MyThread<'a> {
    sink: ExtEventSink,
    receiver: Receiver<Request>,
    device: Option<SendWrapper<Box<dyn DeviceTrait>>>,
    stream: Option<SendWrapper<Box<dyn 'a + for<'b> Stream<'b, Item = ImageView<'b>>>>>,
}

impl<'a> MyThread<'a> {
    fn new(sink: ExtEventSink, receiver: Receiver<Request>) -> Self {
        Self {
            sink,
            receiver,
            device: None,
            stream: None,
        }
    }

    fn run(&mut self) {
        loop {
            // handle next frame
            if let Some(stream) = &mut self.stream {
                // Grab next webcam frame
                let image_view = stream.next().expect("failed to get frame");

                // Convert to druid's ImageData
                // FIXME: can we reduce the amount of conversions here?
                let ffi_image_buffer = ImageBuffer::from(&*image_view);
                let image_buffer: image::ImageBuffer<image::Bgra<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(
                        ffi_image_buffer.width(),
                        ffi_image_buffer.height(),
                        ffi_image_buffer.raw().as_slice().unwrap().to_vec(),
                    )
                    .expect("Failed to convert ffimage::ImageBuffer to image::ImageBuffer");
                let dynamic_image = image::DynamicImage::ImageBgra8(image_buffer);
                let image_data = ImageData::from_dynamic_image_with_alpha(dynamic_image);

                // Send to render thread
                self.sink
                    .submit_command(IMAGE_DATA, image_data, None)
                    .expect("failed to submit command");
            }

            // calculate recv timeout
            let timeout = match self.stream {
                Some(_) => std::time::Duration::from_millis(0),
                // iterate more slowly if we're not streaming
                None => std::time::Duration::from_millis(100),
            };

            // receive requests
            if let Ok(request) = self.receiver.recv_timeout(timeout) {
                match request {
                    _ => println!("ignored request"),
                }
            }
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(image_data) = cmd.get(IMAGE_DATA) {
            data.buffer = Some(Arc::new(image_data.clone()));
        }

        // stop: remove current frame, send stop message to thread

        // start: send start message to thread
        true
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let image = immediate::Immediate::new(|data: &AppState| {
        let component = if let Some(image) = data.buffer.clone() {
            Image::new(ImageData::clone(&image))
                .fill_mode(FillStrat::FitWidth)
                .boxed()
        } else {
            SizedBox::empty().boxed()
        };

        Some(component)
    });

    let start = Button::new("Start").on_click(|_event, data: &mut AppState, _env| {
        data.connection.start_stream();
    });

    let wrapped_image = SizedBox::new(image).width(200.).height(200.);

    Flex::column().with_child(wrapped_image).with_child(start)
}
fn main() {
    // Create app instance (we need an event sink)
    let main_window = WindowDesc::new(ui_builder).title(LocalizedString::new("Blocking functions"));
    let app = AppLauncher::with_window(main_window);

    // Launch webcam thread
    //let uri = get_uri().expect("Couldn't get URI");
    let sink = app.get_external_handle();
    let (sender, receiver) = channel();

    thread::spawn(move || MyThread::new(sink, receiver).run());

    // Launch app
    let connection = models::Connection::new(sender);
    let state = AppState::new(connection);
    let delegate = Delegate {};
    app.delegate(delegate)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
}
