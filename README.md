# Huhnitor
Intergalactic serial monitor

# Binaries
All pre-compiled binaries can be located <a href="https://github.com/SpacehuhnTech/Huhnitor/releases">here.</a>

# Compiling
Compiling requires you to install <a href="https://www.rust-lang.org/tools/install">rustup</a>.
Clone the repository, cd into the root, and run the command `cargo build --release`, your binary will be located in `target/release`. This process should work on Windows, Mac, and Linux.

# Usage
This serial monitor is designed around being as easy to use as possible, just running the executable, then plugging the device you wish to communicate with when promoted should allow for auto-detection of the correct serial port. If this does not work, or you don't want to use this method for some other reason, specifying `-s` will display the serial ports viewed by the system, then allow you to type in any port path you like.

## Credits
Written by <a href="https://github.com/the-Jamz">Jamz</a> with help from <a href="https://selic.re/">Selicre</a>.

## License 
This software is licensed under the MIT License. See the [license file](LICENSE) for details.  
