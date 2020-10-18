mod immediate;
mod models;

use std::{sync::Arc, thread};

use druid::widget::{FillStrat, Flex, Image, ImageData, SizedBox};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, LocalizedString,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};
use eye::prelude::*;
use ffimage::packed::dynamic::ImageBuffer;

const IMAGE_DATA: Selector<ImageData> = Selector::new("IMAGE_DATA");

struct Delegate {}

#[derive(Clone, Data, Lens)]
struct AppState {
    buffer: Option<Arc<ImageData>>,
}

impl AppState {
    fn new() -> Self {
        Self { buffer: None }
    }
}

// Grab the first device
fn get_uri() -> Option<String> {
    let devices: Vec<models::Device> = eye::device::Device::enumerate()
        .iter()
        .map(|dev| models::Device::from(dev.as_str()))
        .collect();

    if devices.len() > 0 {
        return Some(devices[0].uri.clone());
    }
    return None;
}

fn stream_webcam(sink: ExtEventSink, uri: String) {
    // Grab the device
    let mut device = eye::device::Device::with_uri(&uri).expect("with_uri() error");

    // Set the format
    let mut format = device.format().expect("couldn't read format");
    format.pixfmt = PixelFormat::Bgra(32);
    let format = device.set_format(&format).expect("Couldn't set format");
    if format.pixfmt != PixelFormat::Bgra(32) {
        panic!("device does not support BGRA capture",);
    }

    // Stream video from webcam
    let mut stream = device.stream().expect("stream() error");
    loop {
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
        sink.submit_command(IMAGE_DATA, image_data, None)
            .expect("failed to submit command");
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

    let wrapped_image = SizedBox::new(image).width(200.).height(200.);

    Flex::column().with_child(wrapped_image)
}
fn main() {
    // Create app instance (we need an event sink)
    let main_window = WindowDesc::new(ui_builder).title(LocalizedString::new("Blocking functions"));
    let app = AppLauncher::with_window(main_window);

    // Launch webcam thread
    let uri = get_uri().expect("Couldn't get URI");
    let sink = app.get_external_handle();
    thread::spawn(move || {
        stream_webcam(sink, uri);
    });

    // Launch app
    let state = AppState::new();
    let delegate = Delegate {};
    app.delegate(delegate)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
}
