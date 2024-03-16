[![Tests Workflow Status (main)](https://img.shields.io/github/actions/workflow/status/lostatc/swtchr/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github)](https://github.com/lostatc/swtchr/actions/workflows/test.yaml)

# swtchr

![A screenshot of the swtchr window switcher over a Sway desktop](./etc/screenshot.png)

A pretty Gnome-style window switcher for the [Sway](https://swaywm.org/) window
manager.

Out of the box, you can use `<Super>Tab` and `<Super><Shift>Tab` to page
forward and backward through a list of windows ordered from most to least
recently accessed.

## Installing

You can find prebuilt binaries on the [releases
page](https://github.com/lostatc/swtchr/releases), or you can build from
source.

To build from source, you'll need to install these system dependencies:

- [libgtk-4-dev](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)
- [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell?tab=readme-ov-file#distro-packages)

Then [install Rust](https://www.rust-lang.org/tools/install) and run this
command (you do not need to clone the repo):

```
cargo install swtchr
```

This will install the `swtchr` and `swtchrd` binaries to `~/.cargo/bin/`.

## Getting started

To get started, drop these commands into your Sway config, which is usually
located at `~/.config/sway/config`. Substitute whatever path you installed the
`swtchr` and `swtchrd` binaries to.

```
# Start the swtchr daemon.
exec_always ~/.cargo/bin/swtchrd

# Set up keybinds to open the window switcher.
bindsym $mod+Tab mode swtchr; exec ~/.cargo/bin/swtchr
bindsym $mod+Shift+Tab mode swtchr; exec ~/.cargo/bin/swtchr

# This is important! More information below.
mode swtchr bindsym Backspace mode default
```

See [Configuring swtchr](#configuring-swtchr) to customize the behavior and
keybindings.

See [Styling swtchr](#styling-swtchr) to to customize the appearance.

See [Sway keybinds](#sway-keybinds) to understand what's going on with the
`mode swtchr` part.

See [Using systemd](#using-systemd) to start the swtchr daemon via a systemd
service instead of via your Sway config.

See [Recipes](#recipes) for some examples of different ways you can configure
swtchr.

## Configuring swtchr

You can configure the behavior and keybindings for swtchr in
`~/.config/swtchr/swtchr.toml`. An example config file with sensible defaults
will be generated there the first time you start the swtchr daemon.

The comments in the example config file document what each option does. You can
find it at [src/swtchr.toml](./src/swtchr.toml).

swtchr will look for the `swtchr.toml` file in these places:

1. `$XDG_CONFIG_HOME/swtchr/swtchr.toml`
2. `~/.config/swtchr/swtchr.toml`

## Styling swtchr

You can customize the styling of the window switcher using [GTK
CSS](https://docs.gtk.org/gtk4/css-properties.html). Just drop a CSS file here:

```
~/.config/swtchr/style.css
```

You can look at the default stylesheet [src/style.css](./src/style.css) as an
example.

Additionally, you can open the interactive GTK debugger to inspect objects, see
their CSS classes, and apply CSS styles live:

```shell
env GTK_DEBUG=interactive swtchrd
```

swtchr will look for the `style.css` file in these places:

1. `$XDG_CONFIG_HOME/swtchr/style.css`
2. `~/.config/swtchr/style.css`

## Sway keybinds

You need to configure keybinds in your Sway config to open the window switcher.
All other swtchr keybinds are configured in the [swtchr config
file](#configuring-swtchr).

Let's break down the Sway keybinds we set up in [Getting
started](#getting-started):

```
bindsym $mod+Tab mode swtchr; exec ~/.cargo/bin/swtchr
bindsym $mod+Shift+Tab mode swtchr; exec ~/.cargo/bin/swtchr
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
to escape back to the `default` mode, so you'll need this line as well. You may
need to use this keybind if the swtchr daemon crashes before it's able to
switch back to the `default` mode.

## Using systemd

Rather than start the swtchr daemon via an `exec_always` command in your Sway
config, you may want to use a systemd service instead. This enables
restart-on-failure behavior and makes checking the logs easier.

There is an example systemd unit file provided in
[etc/swtchrd.service](./etc/swtchrd.service). Update the `ExecStart=` line to
match the path you installed the `swtchrd` binary to, and then drop it here:

```
~/.config/systemd/user/swtchrd.service
```

From there, you can run this command to start the swtchr daemon and configure
it autostart when you log into a Sway session:

```shell
systemctl --user enable --now swtchrd.service
```

If your distro doesn't package Sway with a `sway-session.target`, check out
[these
docs](https://wiki.archlinux.org/title/Sway#Manage_Sway-specific_daemons_with_systemd)
on how to roll your own.

## Recipes

Out of the box, swtchr only switches focus to the selected window when you
release `<Super>Tab` / `<Super><Shift>Tab`. You can optionally enable "peeking"
to switch window focus immediately as you cycle through windows in the
switcher.

```toml
# swtchr.toml

[keymap]

#next = "<Super>Tab"
peek_next = "<Super>Tab"

#prev = "<Super><Shift>Tab"
peek_prev = "<Super><Shift>Tab"
```

Instead of switching window focus and closing the window switcher immediately
when you release `<Super>Tab` / `<Super><Shift>Tab`, you can configure swtchr
with separate keybinds to a) switch to the selected window or b) dismiss the
window switcher.

```toml
# swtchr.toml

dismiss_on_release = false
select_on_release = false

[keymap]

dismiss = "Escape"
select = "Return"
```
