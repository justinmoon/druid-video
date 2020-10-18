mod models;

use eye;

fn stream(uri: &str) {
    let mut device = match eye::device::Device::with_uri(&uri) {
        Ok(device) => device,
        Err(e) => panic!("with_uri() error: {:?}", e),
    };

    let mut stream = match device.stream() {
        Ok(res) => res,
        Err(e) => panic!("stream() error: {:?}", e),
    };
    //println!("res: {:?}", res);

    loop {
        match stream.next() {
            Ok(frame) => println!("frame"),
            Err(err) => println!("frame err: {:?}", err),
        }
    }
}

fn main() {
    let devices: Vec<models::Device> = eye::device::Device::enumerate()
        .iter()
        .map(|dev| models::Device::from(dev.as_str()))
        .collect();

    if devices.len() > 0 {
        let uri = devices[0].uri.clone();
        stream(&uri);
    }

    println!("devices: {:?}", devices);
}
