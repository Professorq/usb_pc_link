build:
	cargo build

install: build
	cargo build \
	&& systemctl link ./usb_pc_link.service \
	&& systemctl start usb_pc_link
