use std::process;
use std::sync::mpsc;
use std::time::Duration;
use usb_pc_link::pc_link;
use usb_pc_link::pc_link::{VENDOR_ID, PRODUCT_ID, HandleHotplugChan};

fn main() {
    let context = rusb::GlobalContext::default();

    if !rusb::has_hotplug() {
	panic!("libusb does not support hotplug on this system");
    }

    let (tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

    rusb::HotplugBuilder::new()
	.vendor_id(VENDOR_ID)
	.product_id(PRODUCT_ID)
	.enumerate(true)
	.register(
	    context,
	    Box::new(HandleHotplugChan::from(tx))
	).ok()
	.or_else(|| process::exit(-1));

    loop {
	rusb::devices().unwrap().iter()
	    .filter(|device| pc_link::matches(device))
	    .next()
	    .map(|device| pc_link::connect(device));

	rx.recv();
    }
}
