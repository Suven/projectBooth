sudo apt-get install libudev-dev libusb-1.0-0-dev libfox-1.6-dev

https://github.com/LairdCP/UwTerminalX/wiki/Granting-non-root-USB-device-access-(Linux)

/etc/udev/rules.d/10-photobooth.rules

/etc/udev/rules.d/10-photobooth.rules

```                                                                                            
SUBSYSTEM=="usb", ATTRS{idVendor}=="045e", MODE="0666", GROUP="dialout"
SUBSYSTEM=="usb_device", ATTRS{idVendor}=="045e", MODE="0666", GROUP="dialout"
```

LEDS:
echo 255 | tee /sys/class/leds/*buzz*/brightness

ENABLE_RUNTIME_TESTS=false

libevdev-dev