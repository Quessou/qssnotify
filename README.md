# QSSNotify

## What is it ?

A small CLI project written in Rust that allows to display on your Linux desktop environment using the native notification system.
However, note that it has been tested only on GNOME. It should probably work in a lot of desktop environments, but eh.


## How do you build it ?

Just go into the root of the project and call `cargo run --release`. The binary will be at `${PWD}/target/release/qssnotify`
You need `cargo` (part of the Rust toolchain) to build it.


## How do you use it ?

`qssnotify --help` should give you a lot of information about the interface the CLI provides.
By default, QSSNotify uses `nvim` as its text editor, but you can change it by editing the configuration file located in `~/.qssnotify/config`, You can also change the delay between each notifications in this file.


You must add `qssnotify --daemon` to the list of the programs that are launched automatically when your desktop environment is launched, then QSSNotify will regularly display a notification with one of the sentences you've registered, chosen at random.
