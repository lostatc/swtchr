# swtchr

A Gnome-style window switcher for the [Sway](https://swaywm.org/) window
manager.

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

If you would prefer to install the necessary dependencies using a different
package manager:

- [GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)
- [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell?tab=readme-ov-file#distro-packages)
- [librsvg](https://gitlab.gnome.org/GNOME/librsvg)
