build:
	cargo build

install: build
	cargo build \
	&& systemctl link ./usb_pc_link.service \
	&& systemctl enable usb_pc_link \
	&& systemctl start usb_pc_link
