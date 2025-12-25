# NOTE: Neither exec nor source is sufficient to update loaded plugins; execute this on the command line directly
cargo build && plugin add target\debug\nu_plugin_toon.exe && plugin use toon
