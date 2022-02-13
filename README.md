# rustaria

A Terraria rework in Rust.

# Build instructions

(TODO)

## Linux

If you're running X11, you need to install the development packages of these libraries to build the client:

- `libX11`
- `libXrandr`
- `libXinorama`
- `libXcursor`
- `libXi`
  
Exact instructions differ from distro to distro, and from package manager to another,
but here are some (mostly tested) instructions for the more well-known distros:

### `apt`-based (ex. Debian, Ubuntu)

Install the libraries with this command:

```sh
sudo apt install libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev -y
```

### `yum`-based (ex. Fedora, CentOS)

**Fedora users**: you need to replace every reference to `yum` to `dnf`. It's otherwise identical.

Install the libraries with this command:

```sh
sudo yum install libX11-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel -y
```

### Fedora

The same as [`yum`-based distros](#yum-based-ex-fedora-centos), albeit with `dnf` instead of `yum`:

```sh
sudo dnf install libX11-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel -y
```
