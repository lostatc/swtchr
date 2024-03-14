# swtchr

A pretty Gnome-style window switcher for the [Sway](https://swaywm.org/) window
manager.

Out of the box, you can use `<Super>Tab` and `<Super><Shift>Tab` to page
forward and backward through a list of windows ordered from most to least
recently accessed.

## Getting started

First, install swtchr. You can find prebuilt binaries in the GitHub releases
page, or you can [build from source](#build-from-source).

Next, drop these commands into your Sway config, which is usually located at
`~/.config/sway/config`. Substitute whatever path you installed the `swtchr`
and `swtchrd` binaries to.

```
# Start the swtchr daemon.
exec ~/.local/bin/swtchrd

# Set up keybinds to open the window switcher.
bindsym $mod+Tab mode swtchr; exec ~/.local/bin/swtchr
bindsym $mod+Shift+Tab mode swtchr; exec ~/.local/bin/swtchr

# This is important! More information below.
mode swtchr bindsym Backspace mode default
```

See [Sway keybinds](#sway-keybinds) below to understand what's going on with
the `mode swtchr` part.

## Configuring swtchr

If you want to customize the behavior or keybindings of swtchr, you can copy
the example [swtchr.toml](./swtchr.toml) config file to
`~/.config/swtchr/swtchr.toml`. There are comments documenting each of the
available options.

swtchr will look for that config file in these places:

1. `$XDG_CONFIG_HOME/swtchr/swtchr.toml`
2. `~/.config/swtchr/swtchr.toml`

## Sway keybinds

You need to configure keybinds in your Sway config to open the window switcher.
All other swtchr keybinds are configured in your
[swtchr.toml](#configuring-swtchr).

Let's break down the Sway keybinds we set up in [Getting
started](#getting-started):

```
bindsym $mod+Tab mode swtchr; exec ~/.local/bin/swtchr
bindsym $mod+Shift+Tab mode swtchr; exec ~/.local/bin/swtchr
```

We're using `<Super>Tab` both to open the window switcher and to cycle through
windows once it's open. To prevent Sway from consuming those keypresses once
the window switcher is open, we need to change the [Sway binding
mode](https://i3wm.org/docs/userguide.html#binding_modes). swtchr will
automatically change your binding mode back to `default` when the window
switcher closes.

```
mod swtchr bindsym Backspace mode default
```

Sway only allows you to change the binding mode if you've configured a keybind
to escape back to the `default` mode, so you'll need this line as well. You'll
need to use this keybind if the swtchr daemon crashes before it's able to
switch back to the `default` mode.

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
