# swtchr

A pretty Gnome-style window switcher for the [Sway](https://swaywm.org/) window
manager.

Out of the box, you can use `<Super>Tab` and `<Super><Shift>Tab` to page
forward and backward through a list of windows ordered from most to least
recently accessed.

## Getting started

First, install swtchr. You can find prebuilt binaries in the GitHub releases
page, or you can [build from source](#build-from-source).

Next, drop these commands into your Sway config, which is probably located at
`~/.config/sway/config`. Substitute whatever path you installed the `swtchr`
and `swtchrd` binaries to.

```
# Start the swtchr daemon.
exec ~/.local/bin/swtchrd

# Set up keybinds to open the window switcher.
bindsym $mod+Tab exec ~/.local/bin/swtchr
bindsym $mod+Shift+Tab exec ~/.local/bin/swtchr

# This is important! More information below.
mod swtchr bindsym Backspace mode default
```

## Configuring swtchr

If you want to customize the behavior or keybindings of swtchr, you can copy
the example [swtchr.toml](./swtchr.toml) config file to
`~/.config/swtchr/swtchr.toml`. There are comments documenting each of the
available options.

The keybinds we set up in your Sway config in [Getting
started](#getting-started) are only for opening the window switcher; when the
window switcher is open, they're overridden by the keybinds in your
`swtchr.toml`.

Let's explain the purpose of this line in your Sway config:

```
mod swtchr bindsym Backspace mode default
```

Sway has a concept of keybinding "modes," which are distinct keymaps you can
alternate between. When the window switcher opens, swtchr changes your Sway
mode to one called `swtchr`, and it automatically changes you back to the
default mode when the window switcher closes.

However, Sway requires that all modes have a keybind set up to escape out of
them. To work around this, you need to add one in your Sway config.

## Building from source

This crate requires native dependencies to build.

The easiest way to install these dependencies locally is to clone the repo and
use the provided nix shell. [Install nix](https://nixos.org/download) and then
run:

```shell
git clone https://github.com/lostatc/swtchr
cd ./swtchr
nix-shell
```

You can also use [direnv](https://direnv.net) to load the nix shell
automatically:

```shell
cd ./swtchr
direnv allow
```

If you would rather install the necessary dependencies yourself:

- [GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)
- [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell?tab=readme-ov-file#distro-packages)
- [librsvg](https://gitlab.gnome.org/GNOME/librsvg)

To build the `swtchrd` daemon and `swtchr` client, [install
Rust](https://www.rust-lang.org/tools/install) and run:

```shell
cargo build --workspace --release
```

You can find the generated binaries here:

- `./target/release/swtchr`
- `./target/release/swtchrd`
