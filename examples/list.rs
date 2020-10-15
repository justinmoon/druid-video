
fn main() {

        let devices: Vec<model::device::Device> = eye::Device::enumerate()
                    .iter()
                .map(|dev| model::device::Device::from(dev.as_str()))
                .collect();

}
