# TOON format Nushell plugin

Adds `from toon` and `to toon` commands to Nushell.

```nushell
{a: 1 b: 2} | to toon
```

With these commands in place, Nushell automatically and transparently handles `.toon` files.

```nushell
{a: 1 b: 2} | save test.toon

open test.toon
# => ╭───┬───┬───╮
# => │ # │ b │ a │
# => ├───┼───┼───┤
# => │ 0 │ 2 │ 1 │
# => ╰───┴───┴───╯

open test.toon | describe
# => table<b: int, a: int> (stream)
```

## Plugin Installation

In Nushell, plugins are registered executables. See commands `plugin list`, `plugin add`, `plugin use`.

To install the `nu_plugin_toon` plugin, you can use `cargo` (Rust development toolchain) to build and install your own version, or download a pre-built executable from the latest GitHub release.

See also [Nushell Installing Plugins documentation](https://www.nushell.sh/book/plugins.html#installing-plugins).

### Build and Install Using Cargo

```nushell
cargo install --path . --locked
```

if you have `CARGO_HOME` defined, otherwise the path defaults to `~/.cargo`

```nushell
plugin add $"($env.CARGO_HOME)/bin/nu_plugin_toon.exe"
plugin use toon
```

After add and use, to update the plugin, replacing the executable is enough. To do so, call the `cargo install` command documented above.

## Check Plugin Status

```nushell
plugin list | where name == 'toon' | table -e
```
