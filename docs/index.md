# projectBooth

projectBooth is a piece of software you can run on a Raspberry Pi to build
your own photobox. 

Because of its modularity it is possible to start simple, with just a camera
and trigger, or build more complex setups with one or multiple displays to 
show countdowns behind a magic mirror, the latest took photos, a download-gallery
for your guests and a printer.

**projectBooth is currently work in progress and not ready to use.**

## Example build
TODO

## Functionality

### Components
![architecture](./images/Architecture@2x.png)

### Legend
![legend](./images/Legend@2x.png)

## Shopping List

### Raspberry Pi <small>needed</small>
Most units with enough USB-Ports and a WLAN-Chip should work. That said, projectBooth was currently only tested on a [Raspberry Pi 4 Model B](https://amzn.to/2U8DFnR)[^1].

### Camera <small>needed</small>
A list of supported cameras (some with hints on how to start them) can be found on the homepage of [gphoto](http://www.gphoto.org/doc/remote/). The [reference setup](#example-build) was tested with a [Canon Powershot G7X Mark II](https://amzn.to/2t3Febu)[^1].

### Trigger <small>needed</small>
To trigger capturing of a photo / printing, you will need some kind of input. In theory projectBooth should support both, the wired and the wireless versions of the Buzz-Contollers for the PS2. The [reference setup](#example-build) was only tested with the [wired ones](https://amzn.to/2uADruE)[^1].

### Mirror <small>optional</small>
If you want your box to have a mirror as in the reference setup, you can achieve that by either buying a one-way mirror, or a applying a [mirror foil](https://amzn.to/30UPZcm)[^1] to a [plexiglas plate](https://amzn.to/2RvQqal)[^1].

The foil might be a little bit tricky to apply and darkens both, the mirror and the picture the camera takes a bit. On the other side it's magnitudes cheaper. You can see this variant in the [reference setup](#example-build).

If you decide you want to use a mirroring foil you also have to make sure that the inside of your box is much darker then the environment outside of the box, as more light is able to pass through the foil from dark to bright.

### Printer

*TODO*

## Setup
TODO

## FAQ

### I made one! Do you want to see it?
Definitly! Don't hesistate sending pictures, videos, or even just field reports in via an [issue on Github](https://github.com/Suven/projectBooth/issues).

### Do you rent boxes?
Nope.

## Disclaimer
projectBooth is written as a hobby project. I developed it using rust, while learning the language. Therefore you should **not** have to high hopes/expectations on stability or security.  

[^1]: This is a link to amazon which contains a ref-attribute.