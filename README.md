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

### Using released binary (Recommended for Windows)

1. Go to the [release page](https://github.com/SpacehuhnTech/Huhnitor/releases) and download a binary for your OS from the latest release.
2. Run it by simply double clicking it or via terminal `./huhnitor` or `sudo ./huhnitor`  
   **Linux & Mac** users will have to make the binary executable first by running `sudo chmod +x huhnitor`
3. [Optional] Add it to the `PATH` variable for easy use in the terminal

### Using Snap (Recommended for Linux)

1. [Install snap](https://snapcraft.io/docs/installing-snapd) if it doesn't already come with your Linux distribution.
2. Open a terminal and type  
   `sudo snap install huhnitor --edge --devmode`  
3. To start simply run `sudo huhnitor` in a terminal

If you get a `huhnitor not found` message, try adding snap to the PATH by running `export PATH="$PATH:/snap/bin"`.  

### Using Homebrew (Recommended for macOS)

1. Make sure [Homebrew](https://brew.sh/) is installed
2. Open a terminal and type  
   `brew tap spacehuhntech/huhnitor`  
   `brew install huhnitor`  
   or as a one-liner: `brew tap spacehuhntech/huhnitor && brew install huhnitor`
3. To start simply run `huhnitor` in a terminal

**Pro tip**: Homebrew can also be installed on Linux, and the Windows Subsystem for Linux.

### Compiling it yourself

Precompiled binaries can be found at [releases](https://github.com/SpacehuhnTech/Huhnitor/releases).  
But if you want, you can compile the Huhnitor yourself:  

1. Install Rust using [rustup](https://www.rust-lang.org/tools/install)  
   **Linux users** will also need to run `sudo apt install libudev-dev pkg-config`  
   **Windows users** have to install [Visual C++ Build Tools 2019](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019)  
2. [Download and unzip](https://github.com/SpacehuhnTech/Huhnitor/archive/master.zip) or `git clone https://github.com/SpacehuhnTech/Huhnitor.git` this repository
3. In the root directory of the repo run `cargo build --release`, your binary will be located in `target/release/`

## Usage

The Huhnitor is designed to be as easy to use as possible:  

1. Open huhnitor
2. Plug in your deauther
3. Have fun using the command line interface of the [ESP8266 Deauther](https://github.com/SpacehuhnTech/esp8266_deauther) :slightly_smiling_face:  

If the huhnitor has issues connecting to your deauther, try running it as administrator or via `sudo huhnitor`.  
You can also give a user permission to access the serial ports by running `sudo usermod -a -G dialout <username>`.  

The Huhnitor can run scripts (a series of pre-written commands) if you enter `huhn read [filename]` once you are connected to a deauther. The file paths are relative to your current command line location (not the executable's) and are essentially a series of newline separated deauther commands.

To stop running a command on the deauther, you can hit ctrl + c, which is in theory more convenient and should help to prevent accidental disconnects. This does, however, also mean that you cannot exit the Huhnitor with ctrl + c, therefore once a serial connection has been opened, entering `exit` must be used to exit the Huhnitor.

### Arguments

| Argument              | Description                    |
| --------------------- | ------------------------------ |
| `--help`     or `-h`  | print this help screen         |
| `--driver`   or `-d`  | open driver page               |
| `--no-auto`  or `-a`  | disable automatic port connect |
| `--no-color` or `-c`  | disable colored output         |

## Drivers

Your deauther is not detected when plugged in?  
**Make sure the USB connection is working. Some cables can only charge but not transmit data.**  
Depending on the serial chip that is used on the ESP8266 development board you have, you might need to install the right driver:  

* [CP210x](https://www.silabs.com/products/development-tools/software/usb-to-uart-bridge-vcp-drivers)
* [CH341](http://www.wch-ic.com/search?q=cH341&t=downloads)
* [FTDI](https://www.ftdichip.com/FTDrivers.htm)

Not sure which one to install? A lot of ESP8266 based development boards use a chip from the CP210x family, try starting there.   

## Credits

Made with :heart: by [Jamz](https://github.com/the-Jamz) with help from [Selicre](https://selic.re)<br>
in cooperation with [Spacehuhn Technologies](https://github.com/SpacehuhnTech/)

## License

This software is licensed under the MIT License. See the [license file](LICENSE) for details.  
