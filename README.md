usb_pc_link
===========

Complete the connection protocol with Philips USB HiFi on Linux and MacOS

It works with Philips MC-M530.

Hotplug Listener Service
-------
Run the usb_pc_linkd daemon as a systemd service: 
```bash
make install
```
When the stereo is plugged in to the usb port or turned on, the usb_pc_linkd sends sends the control
messages that configures it as an audio sink.

(Linux only)

Compile and Run One-off
-------

```bash
cargo build
sudo ./target/debug/usb_pc_link
```

Use
---
Switch on and plug the HiFi. This needs to be run every time after plugging in.

Run as root `./usb_pc_link`

Linux: Choose `Set [Philips Audio Set]` as sound card (alsa or pulseaudio)

Linux Debian 12 (pipewire):
* Three output devices appear after plugging in:
    - `Digital Output (S/PDIF) - Philips Audio Set`: does not work
    - `Analog Output - No Bass Boost - Philips Audio Set`: works
    - `Analog Output - Bass Boost - Philips Audio Set`: works
* The play, pause, forward and backward buttons from the Philips to control Debian work
* Cannot use the volume from Debian to control volume on the Philips (but volume on the Philips works)

MacOS: Choose `Philips Audio Set` as Output Device

Working
-------
The snd-usb-audio Linux module lacks something that the Widows driver do. Appearently, the Windows driver 'switches on' the USB device on the Micro-System.
I find two usb packet that seems to do it.

    40 04 00 00 a4 ef 01 00 01
    |  |  |     |     |     |
    |  |  value index |     data
    |  request        length
    requesttype


    40 04 00 00 a4 f0 01 00 ff
    |  |  |     |     |     |
    |  |  value index |     data
    |  request        length
    requesttype

`usb_pc_link` uses VendorID and ProductID to select the USB device. If these IDs are not 0471:0111 (see output `lsusb`), you may change the lines :

    #define VENDOR_ID  0x0471
    #define PRODUCT_ID 0x0111

Thanks
===========
This is a simple port of Henri D's [usb_pc_link](https://github.com/nheir/usb_pc_link)

With all libusb interop wrapping provied by [rusb](https://github.com/a1ien/rusb)
