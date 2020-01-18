# projectBooth

**Work in Progress**

This will be a rust-application that can be run on a raspberry pi in combination with a camera and a PS2 buzz-controller.

You could use it as a starting point to build your own magic mirror / photobooth.

## Dev-Hints

To build / run this software you will need to meet some requirements.

* lib-dependencies `sudo apt-get install libevdev-dev`
* access to usb-devices `sudo cp 10-photobooth.rules /etc/udev/rules.d/10-photobooth.rules`
* add your user to dialout `sude usermod -a -G dialout YOURUSERNAME`

If you have issues configuring your user-rights it might be easier to

* `cargo build`
* `sudo ./target/debug/project_booth`
