mod models;

use eye;

fn main() {
    let devices: Vec<models::Device> = eye::device::Device::enumerate()
        .iter()
        .map(|dev| models::Device::from(dev.as_str()))
        .collect();
    println!("devices: {:?}", devices);
}
