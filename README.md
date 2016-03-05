I'm teaching myself embedded programming using Rust, and this repository contains the code I produce. Please check out the accompanying blog at http://embedded.hannobraun.de/.

The code in here is written in Rust and runs on the Arduino Due, which has a 32-bit ARM microcontroller (Atmel SAM3X8E) on it.

After cloning the repository, follow these steps to get it working:
- Run `git submodule update --init`
- Run `./build-dependencies`

You need a recent nightly version of Rust[1]. The version needs to match the version of the submodule in vendor/rust. Sorry for that inconvenience, but right now I don't really know how to structure it better.

There are also some other things you need:
- arm-none-eabi-gcc: This is a cross-compiler for ARM bare-metal targets. How to get that is platform-dependent, but I'm sure Google will help you out.
- BOSSA[2]: This is required to upload the program to the Arduino. Again, Google should help you with the installation for your platform.

Once you're all set up, you can compile the program by running `./compile`. You can compile and upload it to the Arduino in one step by calling `./upload`.

Detailed step-by-step instructions are available on the blog:
http://embedded.hannobraun.de/


[1] Rust: https://www.rust-lang.org/
[2] BOSSA: http://www.shumatech.com/web/products/bossa
