# TRI

[![tests](https://github.com/WhiteBlackGoose/tri/actions/workflows/tests.yml/badge.svg)](https://github.com/WhiteBlackGoose/tri/actions/workflows/tests.yml)
[![Install via Nix](https://img.shields.io/badge/Install%20via%20Nix-7eb7e2?logo=nixos&style=flat-square&labelColor=4c6eb4&logoColor=white)](https://nixos.org/download.html)
[![Built with Rust](https://img.shields.io/badge/Built%20with%20Rust-F49300?logo=rust&style=flat-square&labelColor=F74C00&logoColor=white)](https://www.rust-lang.org/)

`TRI` is like git but for image manipulation.

## Usage

First, initialize the repo.
```
tri init --path image.png
```

Now you can make commits, e. g.:
```
tri commit crop 50x50+0x0
```

Check logs:
```
tri log
```

Print commit tree:
```
tri tree
```


## Installation

### Most users

#### 1. Install nix [**Nix official instruction**](https://nixos.org/download.html)

#### 2. Run installation
```
nix profile install github:WhiteBlackGoose/tri \
--extra-experimental-features 'nix-command flakes'
```

#### 3. Enable tab completion
```
export XDG_DATA_DIRS="~/.nix-profile/share:$XDG_DATA_DIRS"
```
to your `.bashrc` (or your shell's corresponding file).

### NixOS flake users

If you're a NixOS user, add this repo as flake input:
```nix
tri-input.url = "github:WhiteBlackGoose/tri";
```
Then you can add it as module:
```nix
{ environment.systemPackages = [ tri-input.packages.${system}.default ]; }
```

### Manual installation

You will need Rust, imagemagick. Build the repo, and optionally install man pages and shell completions from ./artifacts.

## Documentation

If you run Nix, you can simply call

```
man tri
```

If you do not run Nix on Linux, you can see the man-page manually by calling the following command in the project folder after you built the project (using cargo build):

```
man ./artifacts/tri.1
```

Or refer to [web version](https://whiteblackgoose.github.io/tri/).
