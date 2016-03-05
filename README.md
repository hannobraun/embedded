## Introduction

I'm teaching myself embedded programming using Rust, and this repository contains the code I produce. Please check out the accompanying blog at http://embedded.hannobraun.de/.

The code in here is written in Rust and runs on the Arduino Due, which has a 32-bit ARM microcontroller (Atmel SAM3X8E) on it.


## Prerequisites

You need the following software to use this repository:
- A nightly version of [Rust](http://rust-lang.org), preferably the exact same version that is checked out in `vendor/rust`. If this repository has become out of date or you want to use a different Rust version for some other reason, please read [Changing the Rust Version](#changing-the-rust-version) below.
- `arm-none-eabi-gcc`: This is a cross-compiler for ARM bare-metal targets. How to get that is platform-dependent, but I'm sure Google will help you out.

I'm developing and testing on Linux exclusively (Arch Linux, at the moment). I don't think there's any fundamental reason why it wouldn't work on other platforms, though. Pull requests welcome!


## Compiling and Uploading

After cloning this repository, run the following command:
`git submodule update --init`

This will check out the included Rust version. You only need to do this once, to set the repository up.

The blink program is located in the `blink/` directory. You can compile that program by running `./compile` from the root directory.

To upload the program to the Arduino Due, run `./upload` from the root directory. The `upload` script calls `compile`, so you don't need to run `./compile` manually before every upload.

Please note that the `upload` script assumes the microcontroller is connected as `/dev/ttyACM0`. This works on my Arch Linux system, but it may not work on your system. Some Linux systems always have multiple `/dev/ttyACM*` files, whether something is connected or not. One of them should work for you.


## Changing the Rust Version

When you compile the code in this repository, two versions of Rust will be used:
1. The compiler will be the Rust version installed on your system.
1. The core library will be taken from the Rust version in `vendor/rust`.

The reason for this is, that the code needs to be cross-compiled for another platform, but Rust only includes libraries for the platform that it runs on.

Both versions should match as closely as possible, otherwise it may not compile, or possibly even cause other bugs. To change the version used for `libcore`, do the following:
1. Go to the Rust repository: `cd vendor/rust`
1. Pull the latest changes: `git pull`
1. Check out the version you want to use: `git checkout <VERSION>`

To check out the version that matches your installed version of Rust, run `rustc --version`. The output should look something like this:
`rustc 1.9.0-nightly (339a409bf 2016-03-01)`

The relevant part of that version is `339a409bf`, as it identifies the commit that corresponds to the version. To check out that version, run: `git checkout 339a409bf`.


## More Information

Detailed step-by-step instructions are available on the blog:
http://embedded.hannobraun.de/
