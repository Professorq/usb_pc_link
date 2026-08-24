
use rusb::UsbContext;
use std::process;
use std::time::Duration;
use usb_pc_link::pc_link::{VENDOR_ID, PRODUCT_ID, HandleHotplug};

fn main() {
    let context = rusb::GlobalContext::default();
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

