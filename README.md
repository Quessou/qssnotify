# QSSNotify

## What is it ?

A small CLI project written in Rust that allows to display on your Linux desktop environment using the native notification system.
It is not suited for regular timed reminders like you'd expect from an agenda or a To-do list, but more thought to display regular messages (Soothing stuff, reminders to take breaks or take care of yourself).

## Which OSes and desktop environments are supported ?

It has been tested only on GNOME. It should probably work in a lot of desktop environments since QSSNotify depends on a crate to handle this part.
It may also work on Windows...


## How do you build it ?

Just go into the root of the project and call `cargo run --release`. The binary will be at `${PWD}/target/release/qssnotify`
You need `cargo` (part of the Rust toolchain) to build it.


## How do you use it ?

`qssnotify --help` should give you a lot of information about the interface the CLI provides.
By default, QSSNotify uses `nvim` as its text editor, but you can change it by editing the configuration file located in `~/.qssnotify/config`, You can also change the delay between each notifications in this file.


You must add `qssnotify --daemon` to the list of the programs that are launched automatically when your desktop environment is launched, then QSSNotify will regularly display a notification with one of the sentences you've registered, chosen at random.

* `qssnotify --add`             : Opens a text editor so that you can write the sentence you want to be registered. When leaving the editor, the written sentence will be saved internally.
* `qssnotify --list`            : Lists all registered sentences with their associated hash.
* `qssnotify --edit <HASH>`     : Edits the sentence whose hash is given in parameter in a text editor.
* `qssnotify --delete <HASH>`   : Deletes the sentence whose hash is given in parameter
