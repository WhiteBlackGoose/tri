# TRI

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

Install [nix](https://nixos.org/download.html) (*the* amazing package manager).

Then run
```
nix profile install github:WhiteBlackGoose/tri \
--extra-experimental-features nix-command \
--extra-experimental-features flakes
```

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
