# TRI

[![tests](https://github.com/WhiteBlackGoose/tri/actions/workflows/tests.yml/badge.svg)](https://github.com/WhiteBlackGoose/tri/actions/workflows/tests.yml)

`TRI` is like git but for image manipulation.

## Usage

First, initialize the repo.
```
tri init --path image.png
```

Now you can make commits using any ImageMagick command, e. g.:
```
tri commit crop 50x50+0x0

tri commit monochrome

tri commit rotate 90
```

Check logs:
```
tri log
```

Print commit tree:
```
tri tree
```

Reset the HEAD to another commit by specifying the unique first few characters of the hash (run tri tree or log to see the hashes):
```
tri reset 9f
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
modules = [
{
  environment.systemPackages = [
    tri-input.packages.${system}.default
  ];
}
...
```


## Contributing

To start developing this project, get `nix` package manager and run
```
nix develop
```
In the root of the repo.

To try out the app, run
```
nix shell
```

Shell completion is only available for 
