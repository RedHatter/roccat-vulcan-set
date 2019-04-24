# roccat-vulcan-set

A simple command line utility to set the RGB values on the [Roccat Vulcan 120/100](https://en.roccat.org/Keyboards/Vulcan) keyboard.

## Usage

Takes a list of keycodes followed by a color value. See [here](https://github.com/RedHatter/roccat-vulcan-set/blob/master/src/util.rs) for a complete list of keycodes. Specify colors by either hex code or three numbers between 0-255. The arguments can be provided on the command line or on stdin. Commands are sent to the keyboard after each newline.

**For performance reasons prefer stdin over command line for long running effects.**

### Examples

#### Stdin

Some example effects have been written in a few languages under the [`/examples`](https://github.com/RedHatter/roccat-vulcan-set/tree/master/examples) folder.

#### Command line

    # Set the space and tab keys to red
    roccat-vulcan-set SPACE 255 0 0 TAB 255 0 0

    # Set numlock to green and keypad slash to blue
    roccat-vulcan-set NUMLOCK #33FF00 KPSLASH 0x000099

### Build and install

You will need the development package for `libhidapi` as well as the rust compiler.

    sudo apt install libhidapi-dev
    curl https://sh.rustup.rs -sSf | sh

From there it's simple to clone, build, and install.

    git clone https://github.com/RedHatter/roccat-vulcan-set.git
    cd roccat-vulcan-set
    cargo install --path .
