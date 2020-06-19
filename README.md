# Huhnitor

<p align="center">
  <img alt="Huhnitor Logo" src="img/logo.png" width="200">
  <br>
  An intergalactic serial monitor for the <a href="https://github.com/SpacehuhnTech/esp8266_deauther/tree/v3">ESP8266 Deauther v3</a>
  <br>
  <img src="https://github.com/SpacehuhnTech/Huhnitor/workflows/Rust/badge.svg?branch=master" alt="Rust badge">
</p>

## Disclaimer

**Please note** that while this software can be used for other serial devices and projects, it is designed to be used with the 
[ESP8266 Deauther Version 3](https://github.com/SpacehuhnTech/esp8266_deauther/tree/v3).  

## Installation 

### Linux
> Coming Huhn

In the meantime, you can download the latest [release](https://github.com/SpacehuhnTech/Huhnitor/releases) for linux and simply run it.

### Mac

1. Make sure [Homebrew](https://brew.sh/) is installed
2. Open a terminal and type  
   `brew tap spacehuhntech/huhnitor`  
    `brew install huhnitor`
3. To run it simply type `huhnitor` in a terminal

### Windows
> Coming Huhn

In the meantime, you can download the latest [release](https://github.com/SpacehuhnTech/Huhnitor/releases) for windows and simply run it.

## Usage

The Huhnitor is designed to be as easy to use as possible:  

1. Open huhnitor
2. Plug in your deauther
3. Have fun using the command line interface of the [ESP8266 Deauther](https://github.com/SpacehuhnTech/esp8266_deauther) :slightly_smiling_face:  

### Arguments

| Argument              | Description                    |
| --------------------- | ------------------------------ |
| `--help`     or `-h`  | print this help screen         |
| `--driver`   or `-d`  | open driver page               |
| `--no-auto`  or `-na` | disable automatic port connect |
| `--no-color` or `-nc` | disable colored output         |

## Drivers

Your deauther is not detected when plugged in?  
**Make sure the USB connection is working. Some cables can only charge but not transmit data.**  
Depending on the serial chip that is used on the ESP8266 development board you have, you might need to install the right driver:  

* [CP210x](https://www.silabs.com/products/development-tools/software/usb-to-uart-bridge-vcp-drivers)
* [CH341](http://www.wch-ic.com/search?q=cH341&t=downloads)
* [FTDI](https://www.ftdichip.com/FTDrivers.htm)

Not sure which one to install? A lot of ESP8266 based development boards use a chip from the CP210x family, try starting there.   

## Compiling

Precompiled binaries can be found at [releases](https://github.com/SpacehuhnTech/Huhnitor/releases).  
But if you want, you can compile the Huhnitor yourself:  

1. Install Rust using [rustup](https://www.rust-lang.org/tools/install)  
   **Linux users** also need to install libudev by running `sudo apt install libudev-dev pkg-config`  
   **Windows users** have to install [Visual C++ Build Tools 2019](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019)  
2. [Download and unzip](https://github.com/SpacehuhnTech/Huhnitor/archive/master.zip) or `git clone https://github.com/SpacehuhnTech/Huhnitor.git` this repository
3. In the root directory of the repo run `cargo build --release`, your binary will be located in `target/release/`

## Credits

Made with :heart: by [Jamz](https://github.com/the-Jamz) with help from [Selicre](https://selic.re)<br>
in cooperation with [Spacehuhn Technologies](https://github.com/SpacehuhnTech/)

## License

This software is licensed under the MIT License. See the [license file](LICENSE) for details.  
