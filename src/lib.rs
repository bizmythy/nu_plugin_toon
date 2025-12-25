use nu_plugin::{Plugin, PluginCommand};
mod from_toon;
mod nu_to_toon;
mod toon_to_nu;
mod to_toon;

use crate::from_toon::FromToon;
use crate::to_toon::ToToon;

pub struct ToonPlugin;

impl Plugin for ToonPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromToon), Box::new(ToToon)]
    }
}
