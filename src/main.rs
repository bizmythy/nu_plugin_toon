use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_toon::ToonPlugin;

fn main() {
    serve_plugin(&ToonPlugin, MsgPackSerializer {})
}
