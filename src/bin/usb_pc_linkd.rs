
use rusb::UsbContext;
use std::process;
use std::time::Duration;
use usb_pc_link::pc_link::{VENDOR_ID, PRODUCT_ID};
use usb_pc_link::pc_link_async::HandleHotplug;

fn main() {
    let context = rusb::GlobalContext::default();

    if !rusb::has_hotplug() {
	panic!("libusb does not support hotplug on this system");
    }

    rusb::HotplugBuilder::new()
	.vendor_id(VENDOR_ID)
	.product_id(PRODUCT_ID)
	.enumerate(true)
	.register(
	    context,
	    Box::new(HandleHotplug{})
	).ok()
	.or_else(|| process::exit(-1));

    loop {
	context.handle_events(Some(Duration::from_secs(10)))
	    .unwrap();
    }
}

