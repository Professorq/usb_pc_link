use rusb;
use std::process;
use std::time::Duration;

pub const VENDOR_ID: u16 =  0x0471;
pub const PRODUCT_ID: u16 = 0x0111;

/**
 * Connection.
 *
 * send
 * 40 04 00 00 a4 ef 01 00 01
 * |  |  |     |     |     |
 * |  |  value index |     data
 * |  request        length
 * requesttype
 *
 * send
 * 40 04 00 00 a4 f0 01 00 ff
 * |  |  |     |     |     |
 * |  |  value index |     data
 * |  request        length
 * requesttype
 */
const REQUEST_TYPE: u8 = 0x40;
const REQUEST: u8 = 0x04;
const VALUE: u16 = 0x00;
const INDEX: [u16; 2] = [0xa4ef, 0xa4f0];

pub fn matches<T: rusb::UsbContext>(device: &rusb::Device<T>) -> bool {
    let descriptor = device.device_descriptor();
    match descriptor.ok() {
	Some(desc) =>
	    desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID,
	None => false
    }
}

pub fn connect<T: rusb::UsbContext>(device: rusb::Device<T>) {

    let handle = device.open()
	.unwrap_or_else(|e| {
	    let desc = device.device_descriptor().unwrap();
	    panic!("Device {}:{} found but failed to open: {}", desc.vendor_id(), desc.product_id(), e);
	});
    let duration = Duration::from_millis(1000);

    handle.write_control(REQUEST_TYPE, REQUEST, VALUE, INDEX[0], &[0x01], duration)
	.unwrap_or_else(|_| process::exit(-3));
    handle.write_control(REQUEST_TYPE, REQUEST, VALUE, INDEX[1], &[0xff], duration)
	.unwrap_or_else(|_| process::exit(-4));
}

pub struct HandleHotplug {
}

impl<T: rusb::UsbContext> rusb::Hotplug<T> for HandleHotplug {
    fn device_arrived(&mut self, device: rusb::Device<T>) {
	connect(device);
    }

    fn device_left(&mut self, _device: rusb::Device<T>) {
	// Nothing to do
    }
}
